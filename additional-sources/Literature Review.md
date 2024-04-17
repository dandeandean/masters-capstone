## C2Rust  

C2Rust is a designed with the express purpose of generating a usable Rust implementations from code written in C [c2rust](https://c2rust.com/). To that end, the project is not concerned with implementing the C code in safe Rust, rather it is principally concerned with the primary translation between the languages. It succeeds in this goal. It achieves this through extensive use of unsafe Rust, which allows the automatic translation to mirror exactly what the C code is doing, barring some limitations [c2rust limitations](https://github.com/immunant/c2rust/wiki/Known-Limitations-of-Translation). 

The next logical step is to transform the unsafe Rust into safe Rust if it is possible. This is not trivial.

Research:
- [[improving-c2rust.pdf]] -- static analysis
- [[rust-galeed.pdf]] -- identify intra-language interaction bugs
- [[ownership-guided.pdf]] -- tools to infer ownership
- [[RUSTY.pdf]] -- identifies three main limitations
- [[in-rust-we-trust.pdf]]
- [[lock-and-key.pdf]] ❌

##  Use of LLMs


Recently, the programming world has been adopting Large Language Models (LLMs) to aide in programming. 
This is a very common use of LLMs, however it poses some problems. 
Chiefly, the current generation of LLMs is extremely sensitive to slight differences in prompting [[llm-prog-ass.pdf]] [[llm-impact.pdf]].
This is only compounded by the design of LLMs, in which they sample from distribution of responses. 
LLMs have also been investigated in work more pertinent to this paper--fixing Rust compilation errors.
RustAssistant is a tool that leverages LLMs to solve Rust compilation problems [[llm-fix-rust.pdf]]. 
This paper finds a significant increase in success in solving compilation errors from GPT-3 to GPT-4 [[llm-fix-rust.pdf]], implying that this tool will only become more prevalent in the future of software development. 
This LLM powered translation is not perfect, however. 
Large real-world projects offer problems for LLMs as the amount of context to consider could be massive, and some is unrepresentable, such as file structure [[llm-introduce-bugs.pdf]].
Overall the main problem with LLMs is their unpredictability and relatively high degree of randomness as opposed to traditional translation tools. 
One possible method to remedy this is the use of formal verification tools such as Aeneas [[AENEAS.pdf]] to prove correctness at each end of the translation process. 


Sources:
- [[llm-prog-ass.pdf]]
- [[llm-impact.pdf]]
- [[llm-fix-rust.pdf]]
- [[llm-introduce-bugs.pdf]]
- [[AENEAS.pdf]] -- formal verification 

## C++ plus
In this section, we investigate the development  of C++ security mechanisms as an alternative to transferring away from the C language family.

Many attempts at C/C++ memory safety have been attempted. 
Some projects include SafeC, CCured, Cyclone, and Iron Clad C++ [[close-to-safe-cpp.pdf]].
Furthermore frameworks to guarantee security at compile time, similar to the borrow checker, have been designed for C++ [[robust-cpp.pdf]].
This is preliminary work, however, as it does not deal with all classes of memory issues. 
Code Pointer Integrity (CPI) is another method concerned with securing function pointers in a C/C++ program. 
There are different levels of guarantees associated with CPI, some of which offer strong amounts of safety guarantees with low amounts of overhead [[code-pointer-tegridy.pdf]]


Unfortunately, however most modern C++ code has not made significant improvements to memory safety from its predecessor C [[close-to-safe-cpp.pdf]].  
The lack of compile-time guarantees is a large problem, as any other method will introduce some amount of overhead during execution. 
More broadly, C++ was designed to be an object oriented version of C, not a safer version of C. 
Rust is capable of sidestepping this issue as it was designed with safety in mind, not as an afterthought. 
This approach to memory safety is much more realistic than being able to refactor existing C/C++ code into Rust, however. 
Furthermore, C/C++ developers could implement some of the ideas from Rust, such as ownership and borrow checking (albeit in a modified form) into their standards moving forward. 
The main benefit of this would be maintaining the status quo of the C family languages being dominant.

Sources:
- [[close-to-safe-cpp.pdf]]
- [[robust-cpp.pdf]]
- [[code-pointer-tegridy.pdf]]
