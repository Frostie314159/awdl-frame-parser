# AWDL frame parser
AWDL(**A**pple **W**ireless **D**irect **L**ink) is a protocol, for wireless P2P communication. AWDL is designed to be low latency and high bandwidth, while requiring only one chip for AWDL and "normal" WiFi. 
This parser can read and write the network frames used by this protocol.
For further information see this [stackoverflow post](https://stackoverflow.com/questions/19587701/what-is-awdl-apple-wireless-direct-link-and-how-does-it-work/28196009#28196009).
## Support
### Frame types
* [x] action
    * [x] MIF
    * [x] PSF
* [ ] data(coming soon)
### TLV 
* [x] Arpa
* [x] Version
* [x] Synchronization parameters
* [x] Channel sequence
* [x] Election parameters
* [x] Election parameters v2
* [x] Synchronization tree
* [x] Data path state
* [x] HT capabilities
* [x] IEEE 802.11 Container
* [x] Service parameters
* [x] Service response
## Features
The parser was designed to be able to run in low flash environments(i.e. ESP32, wasm).
The listed time complexity refers to, if the parser runtime rises with input length. So O(1) means go ahead and O(n) means think before you parse. O(1) parsers are not benchmarked.
TLV | feature | Worst case time complexity
-- | -- | --
Arpa | dns_sd_tlvs | O(n)
Service Parameters | dns_sd_tlvs | O(n)
Service Respone | dns_sd_tlvs | min. O(n)
Channel Sequence | sync_elect_tlvs | O(2n)
ElectionParameters\[V2\] | sync_elect_tlvs | O(1)
Synchronization Parameters | sync_elect_tlvs | O(1)
Synchronization Tree | sync_elect_tlvs | O(n)
HTCapabilities | data_tlvs | O(1)
IEEE80211 Container | data_tlvs | O(n)
Data Path State | data_tlvs | O(n)
Version | version_tlv | O(1)

Do note please, that although the parsers are not yet present, the features are. Also some parsers were bundled into one feature. (For more information on this refer to Milan Stute's [dissertation](https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf#table.caption.42).)
## no_std
The library doesn't require any allocations, due to the author sacrificing parts of his sanity, to use Iterators everywhere.
Allocations are only used for testing, to verify that the reported sizes match reality.
## Credits
Although the actual parser was written by me, the reverse engineering of the AWDL protocol was conducted by Milan Stute and SeeMoo-Lab. So kudos to them...
- https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf
- https://github.com/seemoo-lab/owl
- https://owlink.org/
