# autosshcaler

Custom Kubernetes cluster autoscaler using SSH and WoL.

## Road map

1. Hard coded WoL
1. Hard coded shutdown
1. Hard-coded config articulation
1. API iteration (insecure), still hard-coded config (unsure if REST first)
1. CLAP/environment-based configuration
1. Body-based articulation
1. API security?
1. Manifests
1. Helm chart?
1. Timoni?

## Known issues

- `cargo build` bombs on `build.rs` as there's too many `super`s
  I sortof fixed that now it's dying on not finding prost_types.
  See [open issue](https://github.com/tokio-rs/prost/issues/727)

## Notes

- I haven't checked if Prost use Google's `protoc` under the hood.
  I thnk it does?
  It may not matter that much for a less complex example like this.
  Also "in theory" this is all agnostic soooo....
- I love the idea of a plugin architeture and would like to extend to using
  Intel's fleet management stuff but I'm not convinced there would be _enough_
  outside the plugin to justify the split.
  Plus I'm ages away from that.

## References

- [Prost](https://github.com/tokio-rs/prost)
