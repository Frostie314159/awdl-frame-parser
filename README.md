# AWDL frame parser
## Support
### Frame types
* [x] action
    * [x] MIF
    * [x] PSF
* [ ] data(coming soon)
### TLV 
* [x] Arpa
* [x] Version
* [ ] Synchronization parameters
* [x] Channel sequence
* [ ] Election parameters
* [ ] Election parameters v2
* [ ] Synchronization tree
* [ ] Data path state
* [ ] HT capabilities
* [ ] VHT capibilities
* [ ] Service parameters
* [ ] Service response
## Features
The parser was designed to be able to run in low flash environments(i.e. ESP32, wasm).
To reduce size there is currently the option to disable the "read" or "write" feature, which will strip this functionality from the library. Other than that, there is the option to remove certain TLV parsers from the library, by turning of their respective features.
The listed time complexity refers to, if the parser runtime rises with input length.
TLV | feature | Worst case time complexity
-- | -- | --
Arpa | dns_sd_tlvs | O(n)
Version | version_tlv | O(1)
Channel sequence | sync_elect_tlvs | O(2n)

Do note please, that although the parsers are not yet present, the features are. Also some parsers where bundled into one feature. For more information on this refer to Milan Stute's [dissertation](https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf#table.caption.42).
## no_std
The library is built with the ```alloc``` crate, which makes using it in a no_std environment with an allocator possible.
## Credits
Although the actual parser was written by me, the reverse engineering of the AWDL protocol was conducted by Milan Stute and SeeMoo-Lab. So kudos to them..
- https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf
- https://github.com/seemoo-lab/owl
- https://owlink.org/
