# A7 411-Profiling
Authors: 
    Bryan Solis & Abraham Chango

Acknowledgement of Help:
	The only help we received was when we emailed Dr. Daniels for clarification regarding a few aspects of the profiling assignment. Aside from that, the collaborative work or help was between us, as programming partners. 

What routine (phase) in the final Universal Machine takes up the most time, and
says whether the assembly code could be improved and how:
	We used flamegraph in order to figure out what takes up the most time in our Universal Machine. In our case, we suspect the flamegraph has it written as "libsystem_malloc.dylib' _malloc_zone_calloc" - this function consumes more CPU per execution than other functions and serves to allocate memory dynamically. We suspect this is where our memory is allocated dynamically, which is represented as the heap in our assembly code in seg.rs. We improved the assembly code here by creating a single global variable to initialize the program address at 0. This actually resulted in a 15% boost in performance for midmark.um and sandmark.um, as stated in our A7 lab notebook.
		
Time Spent analyzing the problems posed:
	We spent roughly 3 hours analyzing the problem and looking at exactly how to tackle the assignment. Much of that time was spent refreshing our brains regarding git so that we were using git properly on the homework server, as well as profiling/measuring to see what can be improved in our code.

Time Spent solving the problems:
    We spent roughly 5 hours solving the problems. Much of the time was spent thinking of how to revise the code that can be improved, and then implementing those changes.

Our GitHub Repo: https://github.com/BryanCarloSolis/AS7_Profiling