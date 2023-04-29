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
TLV | feature
-- | --
Arpa | dns_sd_tlvs
Version | version_tlv
Channel sequence | sync_elect_tlvs

Do note please, that although the parsers are not yet present, the features are. Also some parsers where bundled into one feature. For more information on this refer to Milan Stute's [dissertation](https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf#table.caption.42).
## no_std
Execution in no_std environments is possible, as long as an allocator is present.
## Credits
Although the actual parser was written by me, the reverse engineering of the AWDL protocol was conducted by Milan Stute and SeeMoo-Lab. So kudos to them. I would als like to mention [sharksforarms](https://github.com/sharksforarms) the initial creator of the amazing [deku](https://crates.io/crates/deku) crate, which powers the parser.
- https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf
- https://github.com/seemoo-lab/owl
- https://owlink.org/
- https://github.com/sharksforarms/deku
