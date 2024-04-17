What are we trying to accomplish here? 
# Sections
- [[Abstract]]
- [[Introduction]]
- [[Background]]
- [[Examples of Vulnerabilities]]
- [[Literature Review]]
- ~~[[Alternative Solutions]]~~
- [[Conclusion]]
### Problem statement:
There are millions, if not thousands, of lines of C/C++ code that are potentially filled with dastardly memory bugs. We have a safe language that is pretty much just as quick as those seminal languages: [[Rust]], but it is impossible to learn unless you have a Rosh Godal... so that's a problem.
### Potential Solution:
Automatically translate all that C/C++ code into Rust code. 
### Problems with this:
Well it turns out this is not easy at all. This is due to the [[Safety Features in Rust]], which don't allow for manual manipulation of memory in a safe function.
