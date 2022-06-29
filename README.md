# macos-launchd
A simple Safari browser parser (and very simple library) written in Rust!  
Currently this program supports parsing Safari History and Downloads data.  

## Safari History
Safari browser history is stored in a SQLITE file at `/Users/<user>/Library/Safari/History.db`
## Safari Downloads
Safari browser Downloads is stored in a PLSIT file at `/Users/<user>/Library/Safari/Downloads.plist`.  
The PLIST file also contains macOS Bookmark data. This program parses the bookmark data using https://github.com/puffyCid/macos-bookmarks

## References
https://blog.d204n6.com/2021/05/ios-macos-tracking-downloads-from.html
https://forensicswiki.xyz/wiki/index.php?title=Apple_Safari
https://darkdefender.medium.com/brief-introduction-to-macos-forensics-f817c9c83609