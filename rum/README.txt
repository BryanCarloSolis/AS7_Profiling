# 411-Universal Machine
Authors: 
    Bryan Solis & Abraham Chango

Acknowledgement of Help:
	We looked up a couple articles that allowed us to have a better understanding of certain concepts, such as visualizing memory management in depth (helped us understand that we needed an allocated heap). Aside from that, the collaborative work or help received was between us, as programming partners. 

What has been correctly implemented and what has not:
	We believe that some of our module interfaces have been implemented correctly, but the uvm module interface that handles the implementation of our opcodes. The program takes in a file and performs its intended function properly with the correct and desired output, for hello.um and cat.um that is. It does not run correctly with the other UM binaries. 

Briefly enumerates any significant departures from your design:
	We added more module intefaces, specifically we added read_instr, start_program, and uvm (described below) to make our code more readable and work correctly. In turn, some of our original module interfaces were also modified slightly, with the implementation of the opcodes and their functions written in uvm. We also used a dynamically allocated heap to contain our arrays of unsigned 32 bit data and 14 opcodes. 

Architecture of Our Solution:
	As a whole, our Universal Machine has read_instr, instr, segs, regs, uvm, main.rs, and start_program module interfaces.
	Our modules:
	read_instr: this module interface reads the instructions from file and they are read and stored as a sequence of unsigned 32 bit words with big-endian byte order
	instr: this module interface recognizes and defines each instruction. Each instruction is coded by the 4 most significant bits of the instruction word and these bits are called the opcode. This module also handles parsing the instructions to get which registers to work on and the instructions to execute. We store the instructions as an array
	regs: this module interface manages all of the 8 general-purpose registers holding one word each. 
	segs: this module interface l handle all of the memory segments in which each segment contains a sequence of words and is referred to by a distinct 32-bit identifier of the UM.The segments will be identified by their corresponding index One distinguished segment is referred to by the 32-bit identifier 0 and stores the program. This segment is called the ‘0’segment. We will store the memory segments as arrays of words of size 32 bits and those arrays will be the
data structure to represent the memory segments in the UM. The invariant for a memory segment
would be an m = s[i], where s represents the sequence of words of size 32 bits and i represents the
32-bit number that identifies memory segment ‘m’
	uvm: this module interface handles the implementation of the opcodes
	main.rs: this module serves to call the start_program function to run all instructions
	start_program: this module has the start_program function that serves to run all the array of instructions that are stored in an allocated heap


How long it takes our Universal Machine to execute 50 million instructions and how we know:
	We were not able to find out how long it takes our Universal Machine to execute 50 million instructions since our Universal Machine did not run correctly when we ran midmark.
	
	
	
Time Spent analyzing the problems posed:
	We spent roughly 5 hours analyzing the problem and looking at exactly how to tackle the assignment. A majority of that time was spent getting a grasp of the overall file structure. To do this, we analyzed by using the technique of stepwise refinement and composition, where we analyzed each problem by breaking the problem into parts, which in turn can be broken into subparts until the sub-sub-parts are either found in a library or are so easy to be solvable by simple code. 

Time spent preparing our design:
	We spent roughly 3 hours preparing our design. We spent less time preparing the design than analyzing the problems since we already got a grasp of the overall file structure when analyzing. This, in turn, made creating the design easier and take less time.

Time Spent solving the problems:
    We spent roughly 20 hours working on solving the implementation problems and trying to get the Universal machine to work, which took more time than it should. We had trouble understanding some of the concepts as well as getting able to perform the instructions correctly.