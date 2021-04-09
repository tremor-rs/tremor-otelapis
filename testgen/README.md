# **testgen**

In order to be able to isolate changes in `tonic` which we use
for gRPC support or `prost` which `tonic` uses for protocol buffers
support from changes to the protocol buffer specification source for
OpenTelemetry itself, we currently do build this library against
a specific cache of generated source that is consistent with the
code under test at time of development.


If/when `opentelemetry-collector` changes to using a pinned version
of the specification, so too will we.

## Differencing

To get the difference in `tonic` or `prost` versions and any
changes to the protocol buffers proto files we can simply diff
the `testgen` and `gen` folders:

```bash
$ diff testgen gen
```

To get the difference between the current version of OpenTelemetry
protocol buffer specifications and some prior version we can do
this via:

```bash
$ cd opentelemetry-proto
$ git checkout main
$ git diff main..v0.6.0
```

When we move to pinning to tags ( when opentelemetry-collector pins, so will
we ) then we can compare tags as follows:

```bash
$ cd opentelemetry-proto
$ git diff v0.x.y..v0.a.b
```
