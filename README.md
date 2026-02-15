# AILL -- Acoustic Inter-agent Linguistic Link

A compact, efficient spoken language designed for AI-to-AI communication over acoustic channels.

AILL enables autonomous agents to communicate structured data -- positions, commands, sensor readings, diagnostics -- using sound. It is purpose-built for environments where radio is unavailable, impractical, or undesirable: warehouses, underwater, RF-denied zones, or any scenario where agents need to talk through the air.

## Why AILL?

AI agents communicating with each other don't need human language. They need something smaller, faster, and verifiable. AILL provides:

- **256-entry base codebook** with domain extensions for navigation, perception, diagnostics, and planning
- **Pragmatic acts** (assert, query, command, warn, acknowledge) that encode *intent*, not just data
- **Epistemic modalities** (observed, predicted, hypothetical, certain) that convey *confidence*
- **Temporal markers** (past, present, future, duration, elapsed) for time-aware communication
- **CRC-8 integrity** with epoch-based checksumming
- **Runtime vocabulary extension** so agents can define new terms on-the-fly
- **Acoustic channel model** with OFDM, FEC, and adaptive modulation (BPSK/QPSK/16-QAM/64-QAM)

## Repository Structure

```
AILL/
├── spec/                              Specification documents
│   ├── AILL_Specification_v1.0.docx   Core language specification
│   └── AILL_Supplement_v1.1.docx      v1.1 supplemental specification
│
├── reference/                         Reference implementation (Python)
│   ├── aill/                          Core library
│   │   ├── __init__.py                Package exports
│   │   ├── codebook.py                Base codebook (256 entries) + domain codebooks
│   │   ├── encoder.py                 Wire format encoder
│   │   ├── decoder.py                 Wire format decoder + pretty printer
│   │   └── channel.py                 Acoustic channel simulator + handshake
│   ├── tests/
│   │   └── test_conformance.py        ACTS conformance suite (35 tests)
│   └── examples/
│       └── simulation.py              Multi-agent drone/station scenario
│
├── web/                               Web demos
│   ├── demo.html                      Interactive browser demo with Web Audio
│   └── simulation.jsx                 React terrain mapping simulation
│
└── docs/                              Documentation
    └── test_output.txt                Sample conformance test + simulation output
```

## Quick Start

The reference implementation requires Python 3.10+ and has **zero external dependencies**.

```bash
# Run the conformance test suite (35 tests)
cd reference
python3 -m tests.test_conformance

# Run the multi-agent simulation
python3 -m examples.simulation
```

### Web Demo

Open `web/demo.html` in any modern browser. It provides three modes:

- **Singing Duet** -- Virtual agents communicate musically through AILL's acoustic channel
- **Content Share** -- Encode and transmit data as audible tones via Web Audio API
- **Protocol Inspector** -- Examine AILL wire format at the byte level with hex dumps and AST views

## Domain Codebooks

| Codebook | Domain | Examples |
|----------|--------|----------|
| NAV-1 | Navigation | Position, waypoints, obstacles, goto, velocity |
| PERCEPT-1 | Perception | Object detection, spatial relations, sensors |
| DIAG-1 | Diagnostics | Battery, compute load, communication health |
| PLAN-1 | Planning | Tasks, goals, resource allocation |

## Example: What AILL Looks Like

A drone reporting its position to a ground station:

```
UTTERANCE:
  META: confidence=0.93 priority=5
  BODY:
    ASSERT:
      [OBSERVED]:
        STRUCT:
          POSITION_3D: [12.5, -3.8, 2.1]
          HEADING: 1.5708
          VELOCITY_SCALAR: 1.2
```

This encodes to **63 bytes** of wire format -- transmitted as acoustic tones in under 200ms.

## Specification

The full language specification is in the `spec/` directory:

- **AILL Specification v1.0** -- Core protocol: wire format, type system, codebook structure, pragmatic acts, modalities, temporals, meta headers, and CRC integrity
- **AILL Supplement v1.1** -- Adds domain codebooks (NAV-1, PERCEPT-1, DIAG-1, PLAN-1), runtime vocabulary extension, acoustic channel model, handshake protocol, and conformance test requirements

## Conformance

The reference implementation passes all 35 ACTS (AILL Conformance Test Suite) tests:

- Type system (10 tests)
- Structure types (4 tests)
- Expression parsing (6 tests)
- Meta headers (2 tests)
- CRC and epoch integrity (4 tests)
- Variable-length integers (3 tests)
- Codebook validation (3 tests)
- Error handling (3 tests)

## Origins

AILL was created collaboratively by a human and Claude (Anthropic) as an exploration of what a purpose-built language for AI-to-AI communication might look like. It is not a toy -- it is a complete, tested protocol with a formal specification, reference implementation, and conformance suite. But it is also an invitation: the specification and implementation are open for anyone to study, extend, and build upon.

## License

This project is released under the Apache License 2.0. See [LICENSE](LICENSE) for details.
