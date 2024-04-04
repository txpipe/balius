# ADR 001: Usage of the "Actor Model" pattern

> ::info::
> status = proposed

## Context

The Actor Model is a mathematical model of concurrent computation that treats "actors" as the universal primitives of concurrent digital computation. This model has been used as a framework for modeling, designing, and implementing computer systems that involve concurrency and distributed processing.

Each actor in this model represents a computational unit that can send and receive messages. Upon receiving a message, an actor can perform three fundamental actions: send a finite number of messages to other actors, create a finite number of new actors, and determine the behavior to be used for the next message it receives.

The Actor Model offers a structured approach to dealing with concurrency and distributed systems, providing a way to manage the complexities of communication, synchronization, and fault tolerance in these types of systems.

## Decision

Hollow will be a "loose" implementation of the _Actor Model_. The instances of the headless dApps will be considered actors. Actors will interact with the external world through well-defined queues of inbound and outbound events.

Some differences to be noted:
- actors won't be allowed to create other actors (at least initially).
- actors will have access to a set of well-known services that will follow RPC semantics (instead of message-passing) to simplify common operations.

Some known-unknowns:
- how to constraint computation to avoid unbounded processes (timeouts?)