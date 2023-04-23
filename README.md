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
* [ ] Channel sequence
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
To reduce size there is currently the option to disable the "read" or "write" feature, which will strip this functionality from the library.
## no_std
Execution in no_std environments is possible, as long as an allocator is present.
## Credits
Although the actual parser was written by me, the reverse engineering of the AWDL protocol was conducted by Milan Stute and SeeMoo-Lab. So kudos to them.
- https://tuprints.ulb.tu-darmstadt.de/11457/1/dissertation_milan-stute_2020.pdf
- https://github.com/seemoo-lab/owl
- https://owlink.org/
