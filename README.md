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
* [ ] Data path state
* [x] HT capabilities
* [ ] IEEE 802.11 Container
* [x] Service parameters
* [x] Service response
## Features
The parser was designed to be able to run in low flash environments(i.e. ESP32, wasm).
To reduce size there is currently the option to disable the "read" or "write" feature, which will strip this functionality from the library. Other than that, there is the option to remove certain TLV parsers from the library, by turning of their respective features.
The listed time complexity refers to, if the parser runtime rises with input length. So O(1) means go ahead and O(n) means think before you parse. O(1) parsers are not benchmarked.
TLV | feature | Worst case time complexity
-- | -- | --
Arpa | dns_sd_tlvs | O(n)
Service Parameters | dns_sd_tlvs | O(n)
Service Respone | dns_sd_tlvs | min. O(n)
Channel sequence | sync_elect_tlvs | O(2n)
ElectionParameters\[V2\] | sync_elect_tlvs | O(1)
Synchronization Parameters | sync_elect_tlvs | O(1)
Synchronization Tree | sync_elect_tlvs | O(n)
HTCapabilities | data_tlvs | O(1)
Version | version_tlv | O(1)

Do note please, that although the parsers are not yet present, the features are. Also some parsers where bundled into one feature. (For more information on this refer to Milan Stute's [dissertation](https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf#table.caption.42).)
## no_std
The library is built with the `alloc` crate, which makes using it in a no_std environment with an allocator possible.
## Credits
Although the actual parser was written by me, the reverse engineering of the AWDL protocol was conducted by Milan Stute and SeeMoo-Lab. So kudos to them..
- https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf
- https://github.com/seemoo-lab/owl
- https://owlink.org/
