"""
AILL - Acoustic Inter-agent Linguistic Link
Reference Implementation v1.1
"""

from .codebook import (
    FrameControl, TypeMarker, Structure, Quantifier, Logic, Relational,
    Temporal, Modality, Pragmatic, Meta, Arithmetic, Escape,
    BASE_CODEBOOK, DOMAIN_REGISTRY,
    NAV1, PERCEPT1, MANIP1, COMM1, DIAG1, PLAN1, SAFETY1,
)
from .encoder import AILLEncoder, ByteStream, EpochBuilder, crc8
from .decoder import AILLDecoder, pretty_print, UtteranceNode
from .channel import (
    AcousticChannel, ChannelConfig, ChannelStats,
    AgentCapabilities, SessionParams, negotiate_session,
)

__version__ = "1.1.0"
__all__ = [
    'FrameControl', 'TypeMarker', 'Structure', 'Quantifier', 'Logic',
    'Relational', 'Temporal', 'Modality', 'Pragmatic', 'Meta',
    'Arithmetic', 'Escape', 'BASE_CODEBOOK', 'DOMAIN_REGISTRY',
    'NAV1', 'PERCEPT1', 'MANIP1', 'COMM1', 'DIAG1', 'PLAN1', 'SAFETY1',
    'AILLEncoder', 'AILLDecoder', 'ByteStream', 'EpochBuilder',
    'AcousticChannel', 'ChannelConfig', 'AgentCapabilities',
    'SessionParams', 'negotiate_session', 'pretty_print', 'crc8',
]
