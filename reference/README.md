# AILL Reference Implementation v1.1

**Acoustic Inter-agent Linguistic Link** — A spoken language for AI-to-AI communication.

## Overview

This is the reference implementation of the AILL protocol as specified in the AILL Specification v1.0 and Supplemental Specification v1.1. It provides a complete encoder, decoder, acoustic channel simulator, and conformance test suite.

## Project Structure

```
aill_reference/
├── aill/                    # Core library
│   ├── __init__.py          # Package exports
│   ├── codebook.py          # Complete Base Codebook (256 entries) + Domain Codebooks
│   ├── encoder.py           # Wire format encoder (expressions → bytes)
│   ├── decoder.py           # Wire format decoder (bytes → AST)
│   └── channel.py           # Acoustic channel simulator + handshake
├── tests/
│   └── test_conformance.py  # ACTS conformance test suite (35 tests)
├── examples/
│   └── simulation.py        # Multi-agent drone/station scenario
└── README.md
```

## Quick Start

```bash
# Run conformance tests
python -m tests.test_conformance

# Run multi-agent simulation
python -m examples.simulation
```

## Library Usage

```python
from aill import AILLEncoder, AILLDecoder, pretty_print, DOMAIN_REGISTRY

# Encode an utterance
enc = AILLEncoder(agent_uuid=b'\x00' * 16)
enc.start_utterance(confidence=0.95, priority=5)
enc.assert_().observed()
enc.begin_struct()
enc.field(0x0000)  # NAV-1: POSITION_3D
enc.list_of_float32([12.5, -3.8, 2.1])
enc.end_struct()
wire_bytes = enc.end_utterance()

# Decode it
dec = AILLDecoder()
utterance = dec.decode_utterance(wire_bytes)
print(pretty_print(utterance, domain_codebooks=DOMAIN_REGISTRY))
```

## Components

### Codebook (`codebook.py`)
- Complete Base Codebook: all 256 Level 0 entries as Python enums
- NAV-1: Navigation domain (position, waypoints, obstacles, motion commands)
- PERCEPT-1: Perception domain (object detection, spatial relations, sensors)
- DIAG-1: Diagnostics domain (battery, compute, communication health)
- PLAN-1: Planning domain (tasks, goals, resource allocation)

### Encoder (`encoder.py`)
- Fluent API for building AILL utterances
- CRC-8/CCITT implementation
- IEEE 754 float16/32/64 encoding
- Variable-length integer encoding
- Epoch builder with automatic CRC checksums

### Decoder (`decoder.py`)
- Full AST parser for AILL wire format
- Supports all type markers, structures, pragmatic acts, modalities, temporals
- Domain codebook reference resolution
- Pretty printer for human-readable AST output

### Channel Simulator (`channel.py`)
- Acoustic propagation model (distance, temperature, humidity)
- AWGN noise injection based on modulation scheme
- BER calculation for BPSK, QPSK, 16-QAM, 64-QAM
- Convolutional coding gain modeling
- Handshake negotiation with capability discovery
- Channel characterization measurements

## Requirements

- Python 3.10+
- No external dependencies (stdlib only)

## Conformance

The implementation passes all 35 ACTS tests covering:
- Type system (10 tests)
- Structure types (4 tests)
- Expression parsing (6 tests)
- Meta headers (2 tests)
- CRC and epoch integrity (4 tests)
- Variable-length integers (3 tests)
- Codebook validation (3 tests)
- Error handling (3 tests)

## License

This reference implementation is provided for educational and research purposes.
