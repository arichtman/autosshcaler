# autosshcaler

**Suspended**: Moved to Codeberg

Custom Kubernetes cluster autoscaler using SSH and WoL.

## Road map

Series order is approximate.
I reserve the right to get excited once it gets working and skip ahead to something deployable.
Call it dogfooding enthusiasm.

1. Hard coded WoL by MAC
1. Hard coded shutdown by syscall to SSH
1. Hard-coded config articulation (i.e. extracted/centralised but not feed-in)
1. CLAP/environment-based configuration
1. API iteration (insecure), still hard-coded config (unsure if REST first)
1. Body-based articulation
1. Test suite
1. IPv4 IP-to-MAC resolution by ARP
1. IPv6 IP-to-MAC resolution by neighbour/router advertisement/multicast?
1. DNS-to-IP resolution
1. API security?
1. Manifests
1. Helm chart?
1. Timoni?

## Known issues

## Notes

- I haven't checked if Prost use Google's `protoc` under the hood.
  I thnk it does?
  It may not matter that much for a less complex example like this.
  Also "in theory" this is all agnostic soooo....
- I love the idea of a plugin architeture and would like to extend to using
  Intel's fleet management stuff but I'm not convinced there would be _enough_
  outside the plugin to justify the split.
  Plus I'm ages away from that.
- Resolving DNS-to-IP and then IP-to-MAC and the required broadcaast and gateway
  stuff may be a fair bit more manual than I'd expected.
  Where I'm hoping to land is that one only needs to provide the node names as input.
- Actually, I wonder if we could simply use Kubernetes node objects and assume DNS entry of the node name...
- I'm undecided between running a shutdown command via SSH or configuring the user's
  login shell to terminate the machine. The latter feels more secure but is more work.
  Perhaps the answer is a daemonset that's privileged to shut down the node?
- I'm also considering cordoning and draining the node, this would work fine I think
  with a plain service account/clusterrole{binding}.
- The gRPC nature of the call in makes me feel like it should be synchronous and fast,
  but to do this with any safety we probably want at least 10-30 seconds to act.
  I need to research the process in case there's a callback or secondary call we can use.

## References

- [Prost](https://github.com/tokio-rs/prost)
- [libpnet arp example](https://github.com/libpnet/libpnet/blob/main/examples/arp_packet.rs)
- [arp-toolkit crate](https://crates.io/crates/arp-toolkit)
- [trippy docs re CAP_NET_RAW](https://docs.rs/trippy/latest/trippy/#privileges)
- [dns-lookup crate](https://crates.io/crates/dns-lookup)
- [russh example ssh client run command](https://github.com/warp-tech/russh/blob/main/russh/examples/client_exec_simple.rs)
- [wol-rs code](https://github.com/fengyc/wol-rs/blob/main/src/lib.rs)
