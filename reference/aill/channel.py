"""
AILL Reference Implementation - Acoustic Channel Simulator
Acoustic Inter-agent Linguistic Link v1.1

Simulates the physical acoustic channel between AILL agents, including:
- AWGN noise
- Multipath / reverberation
- Frequency-dependent attenuation
- Bit error injection based on SNR and modulation scheme
"""

import math
import random
import struct
from dataclasses import dataclass, field
from typing import Optional


@dataclass
class ChannelConfig:
    """Configuration for the acoustic channel model."""
    snr_db: float = 25.0               # Signal-to-noise ratio in dB
    distance_m: float = 5.0             # Distance between agents in meters
    reverb_rt60_ms: float = 200.0       # Reverberation time (RT60) in ms
    temperature_c: float = 20.0         # Ambient temperature (affects speed of sound)
    humidity_pct: float = 50.0          # Relative humidity
    multipath_taps: int = 4             # Number of multipath reflections
    freq_response_rolloff_db: float = 3.0  # High-freq attenuation per octave above 4kHz
    doppler_shift_hz: float = 0.0       # Relative motion Doppler shift


@dataclass
class ChannelStats:
    """Statistics from a channel transmission."""
    bits_transmitted: int = 0
    bits_errored: int = 0
    ber: float = 0.0                    # Bit error rate
    snr_effective_db: float = 0.0       # Effective SNR after channel effects
    latency_ms: float = 0.0            # Propagation latency
    throughput_bps: float = 0.0         # Effective throughput


# ── Modulation BER curves ────────────────────────────────────────────────

def _q_function(x: float) -> float:
    """Approximation of Q-function (tail probability of standard normal)."""
    if x < 0:
        return 1.0 - _q_function(-x)
    if x == 0:
        return 0.5
    # Abramowitz and Stegun approximation
    t = 1.0 / (1.0 + 0.3275911 * x)
    poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 +
           t * (-1.453152027 + t * 1.061405429))))
    return poly * math.exp(-x * x / 2.0)


def ber_bpsk(snr_linear: float) -> float:
    """Theoretical BER for BPSK."""
    return _q_function(math.sqrt(2 * snr_linear))


def ber_qpsk(snr_linear: float) -> float:
    """Theoretical BER for QPSK (same as BPSK per bit)."""
    return _q_function(math.sqrt(2 * snr_linear))


def ber_16qam(snr_linear: float) -> float:
    """Approximate BER for 16-QAM."""
    return (3.0 / 8.0) * _q_function(math.sqrt(snr_linear * 4.0 / 5.0))


def ber_64qam(snr_linear: float) -> float:
    """Approximate BER for 64-QAM."""
    return (7.0 / 24.0) * _q_function(math.sqrt(snr_linear * 6.0 / 21.0))


# Modulation scheme selection
MODULATION_SCHEMES = {
    'BPSK':   {'bits_per_symbol': 1,  'ber_func': ber_bpsk,   'min_snr_db': 0},
    'QPSK':   {'bits_per_symbol': 2,  'ber_func': ber_qpsk,   'min_snr_db': 10},
    '16-QAM': {'bits_per_symbol': 4,  'ber_func': ber_16qam,  'min_snr_db': 20},
    '64-QAM': {'bits_per_symbol': 6,  'ber_func': ber_64qam,  'min_snr_db': 30},
}


def select_modulation(snr_db: float) -> str:
    """Select the best modulation scheme for the given SNR."""
    if snr_db >= 30:
        return '64-QAM'
    elif snr_db >= 20:
        return '16-QAM'
    elif snr_db >= 10:
        return 'QPSK'
    else:
        return 'BPSK'


# ═══════════════════════════════════════════════════════════════════════
# Channel Simulator
# ═══════════════════════════════════════════════════════════════════════

class AcousticChannel:
    """
    Simulates an acoustic communication channel for AILL.
    
    Models propagation delay, distance attenuation, multipath,
    and bit errors based on channel conditions and modulation.
    """

    def __init__(self, config: Optional[ChannelConfig] = None):
        self.config = config or ChannelConfig()
        self._rng = random.Random()
        self._stats = ChannelStats()

    def seed(self, seed_value: int):
        """Set random seed for reproducible simulations."""
        self._rng.seed(seed_value)

    @property
    def stats(self) -> ChannelStats:
        return self._stats

    def _speed_of_sound(self) -> float:
        """Speed of sound in m/s adjusted for temperature."""
        return 331.3 + 0.606 * self.config.temperature_c

    def _propagation_delay_ms(self) -> float:
        """One-way propagation delay in milliseconds."""
        return (self.config.distance_m / self._speed_of_sound()) * 1000.0

    def _distance_attenuation_db(self) -> float:
        """Free-space acoustic attenuation (inverse square law)."""
        if self.config.distance_m <= 0.1:
            return 0.0
        # Reference distance = 1m
        return 20.0 * math.log10(self.config.distance_m)

    def _effective_snr_db(self) -> float:
        """Compute effective SNR after distance attenuation and reverb effects."""
        snr = self.config.snr_db - self._distance_attenuation_db()
        # Reverb reduces effective SNR slightly (late reflections act as noise)
        if self.config.reverb_rt60_ms > 100:
            reverb_penalty = min(6.0, (self.config.reverb_rt60_ms - 100) * 0.01)
            snr -= reverb_penalty
        return max(snr, -10.0)  # Floor at -10 dB

    def transmit(self, data: bytes) -> tuple[bytes, ChannelStats]:
        """
        Simulate transmission of data through the acoustic channel.
        
        Returns (received_data, stats) where received_data may contain
        bit errors based on channel conditions.
        """
        effective_snr_db = self._effective_snr_db()
        snr_linear = 10.0 ** (effective_snr_db / 10.0) if effective_snr_db > -10 else 0.001
        
        # Select modulation based on effective SNR
        mod_name = select_modulation(effective_snr_db)
        mod = MODULATION_SCHEMES[mod_name]
        
        # Calculate theoretical BER
        theoretical_ber = mod['ber_func'](snr_linear)
        
        # Apply convolutional coding gain (rate-1/2 K=7 Viterbi gives ~5-7 dB coding gain)
        # We model this as reducing the effective BER
        coding_gain_factor = 100.0  # Roughly 2 orders of magnitude BER improvement
        effective_ber = theoretical_ber / coding_gain_factor
        effective_ber = max(0.0, min(1.0, effective_ber))

        # Inject bit errors
        total_bits = len(data) * 8
        bits_errored = 0
        result = bytearray(data)

        if effective_ber > 0:
            for byte_idx in range(len(result)):
                for bit_idx in range(8):
                    if self._rng.random() < effective_ber:
                        result[byte_idx] ^= (1 << bit_idx)
                        bits_errored += 1

        # Calculate throughput
        # Frame duration = 2.5ms, 32 subcarriers
        bits_per_frame = 32 * mod['bits_per_symbol']
        frames_per_sec = 400  # 1/2.5ms
        raw_throughput = bits_per_frame * frames_per_sec
        # Account for rate-1/2 coding and overhead (~70% efficiency)
        effective_throughput = raw_throughput * 0.5 * 0.7

        # Calculate transmission time
        tx_time_ms = (total_bits / effective_throughput) * 1000 if effective_throughput > 0 else float('inf')

        self._stats = ChannelStats(
            bits_transmitted=total_bits,
            bits_errored=bits_errored,
            ber=bits_errored / total_bits if total_bits > 0 else 0.0,
            snr_effective_db=effective_snr_db,
            latency_ms=self._propagation_delay_ms() + tx_time_ms,
            throughput_bps=effective_throughput,
        )

        return bytes(result), self._stats

    def characterize(self) -> dict:
        """
        Perform channel characterization (pre-session measurement).
        Returns measurements that inform parameter negotiation.
        """
        eff_snr = self._effective_snr_db()
        mod = select_modulation(eff_snr)
        
        return {
            'effective_snr_db': round(eff_snr, 1),
            'propagation_delay_ms': round(self._propagation_delay_ms(), 2),
            'distance_attenuation_db': round(self._distance_attenuation_db(), 1),
            'recommended_modulation': mod,
            'reverb_rt60_ms': self.config.reverb_rt60_ms,
            'estimated_ber': MODULATION_SCHEMES[mod]['ber_func'](
                10.0 ** (eff_snr / 10.0) if eff_snr > -10 else 0.001
            ),
            'max_bands': self._recommend_bands(eff_snr),
            'recommended_guard_interval_ms': self._recommend_guard(self.config.reverb_rt60_ms),
        }

    def _recommend_bands(self, snr_db: float) -> str:
        """Recommend band configuration based on SNR."""
        if snr_db >= 30:
            return "B0-B4 (full spectrum)"
        elif snr_db >= 20:
            return "B0-B3 (standard)"
        elif snr_db >= 10:
            return "B0-B2 (reduced)"
        else:
            return "B0-B1 (minimum)"

    def _recommend_guard(self, rt60_ms: float) -> float:
        """Recommend guard interval based on reverberation."""
        if rt60_ms < 100:
            return 0.3  # Default
        elif rt60_ms < 300:
            return 0.5
        elif rt60_ms < 600:
            return 0.8
        else:
            return 1.2


# ═══════════════════════════════════════════════════════════════════════
# Handshake Simulator
# ═══════════════════════════════════════════════════════════════════════

AILL_MAGIC = 0xA111C0DE

@dataclass
class AgentCapabilities:
    """Capabilities of an AILL agent."""
    uuid: bytes = field(default_factory=lambda: bytes(random.getrandbits(8) for _ in range(16)))
    protocol_version: int = 0x0100      # v1.0
    conformance_level: int = 0x02       # Standard
    capabilities_bitmap: int = 0x007F   # Multi-band, QAM64, fountain, RS, adaptive comp, vocab ext
    max_sample_rate_khz: int = 48
    preferred_frame_duration_us: int = 2500
    noise_floor_db_spl: float = 35.0
    codebook_sets: list = field(default_factory=lambda: [0x01, 0x02, 0x05])  # NAV-1, PERCEPT-1, DIAG-1


@dataclass
class SessionParams:
    """Negotiated session parameters."""
    conformance_level: int = 0x01
    modulation: str = "QPSK"
    active_bands: list = field(default_factory=lambda: ["B0", "B1"])
    frame_duration_us: int = 2500
    sample_rate_khz: int = 44
    error_correction: str = "rate-1/2 conv"
    codebook_sets: list = field(default_factory=list)
    sct_max_size: int = 1024


def negotiate_session(agent_a: AgentCapabilities, agent_b: AgentCapabilities,
                      channel: AcousticChannel) -> SessionParams:
    """
    Simulate the AILL handshake and return negotiated parameters.
    Follows the min-common-denominator rule.
    """
    # Conformance level: minimum of both
    conf_level = min(agent_a.conformance_level, agent_b.conformance_level)

    # Capabilities: bitwise AND
    common_caps = agent_a.capabilities_bitmap & agent_b.capabilities_bitmap

    # Sample rate: minimum
    sample_rate = min(agent_a.max_sample_rate_khz, agent_b.max_sample_rate_khz)

    # Frame duration: maximum (slower agent dictates)
    frame_dur = max(agent_a.preferred_frame_duration_us, agent_b.preferred_frame_duration_us)

    # Channel characterization
    ch_info = channel.characterize()

    # Modulation from channel
    modulation = ch_info['recommended_modulation']

    # Bands from channel
    bands = ["B0", "B1"]
    if common_caps & 0x01 and ch_info['effective_snr_db'] >= 20:
        bands.extend(["B2", "B3"])
    if common_caps & 0x02 and ch_info['effective_snr_db'] >= 25:
        bands.append("B4")

    # Error correction
    error_corr = "rate-1/2 conv"
    if common_caps & 0x20:
        error_corr += " + RS(255,223)"
    if common_caps & 0x10:
        error_corr += " + fountain"

    # Common codebook sets
    common_cbs = list(set(agent_a.codebook_sets) & set(agent_b.codebook_sets))

    # SCT size
    sct_size = 1024 if conf_level >= 2 else 64

    return SessionParams(
        conformance_level=conf_level,
        modulation=modulation,
        active_bands=bands,
        frame_duration_us=frame_dur,
        sample_rate_khz=sample_rate,
        error_correction=error_corr,
        codebook_sets=sorted(common_cbs),
        sct_max_size=sct_size,
    )
