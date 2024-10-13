# TA Calculations Tools

Backstory: I became the TA coordinator in F24 for ECE and got some
guidance from the person before me on how to do the role, including
some information about how TA hours are calculated. Part of the goal
of this project is to capture that information in a form that lives
on even when I hand off the role to the next person... and another goal
is to increase transparency of the process.

Why transparency? We are, after all, an organization of collegial governance,
and I think everyone feels better knowing how the TA hours are allocated. While I am
sure that each instructor would be happy to have more hours for their course
in particular, we know that the TA budget is not infinite and also that there
is a limited pool of (qualified) graduate students to fill the available roles.
So now, one and all can read the source code here and see that the calculation
is done in a systematic and fair way.

The calculations done with the current enrollments are always a little bit
on the cautious side. Preliminary enrollment numbers for courses, especially
elective courses, don't reflect what happens in the add/drop period of the term.
If a course's enrollment increases significantly, then after the drop deadline,
an increase for that course is warranted. But if the course enrollment decreases,
we don't "claw back" any TA hours. That would be cruel to the grad student(s) 
affected, but also there's no real mechanism for us to do so (employment law 
and contract law are pretty clear about the contracts we issue).

That said, these calculations just go into a spreadsheet and aren't really the 
final values allocated in the system. There's still manual review and adjustments
for special cases and other things not easily captured in this program.
But for most courses most of the time, this gets the first draft approximately
correct.

Special cases... ah, there are numerous special cases, some of which are
unaccounted for in the code base (as yet! Future improvements). Those are things like...
* Capstone courses (they get 0 TA hours)
* 1st Year Courses (extra hours due to higher needs for support)
* Some lab courses with very strict student:TA ratios (NE cleanroom courses, for example)
* One-off, one-time alterations like new course development.

## Making Changes

Hello there, future TA coordinator. Or maybe I should say, current one,
when I am the one writing from the (ancient) past. I've tried in the design
to make it so that you can easily change the weightings and calculations,
even if you don't (want to) know Rust. 

All the configurable numbers are in `src/config.rs`. You can change any
of them at will and the next run of the program takes those into account. 
No more tutorials (curriculum diet?)? Just set their hours to 0.0, or remove
the `TAHourAllocation` entry from the array that references it. 

I also intentionally made the program chatty on console output about what it
is doing -- e.g., it will say it's adding 12.00 hours to the total for tutorials,
and indicate that the rule is "PER_TERM" (i.e., independent of the number of
students enrolled in the course). It does help with the "how did this number end
up so big?" questions. Any other constants like the min TA threshold are also
defined there. 

(Yes, I could have put these things into a configuration text
file, but that would have been less concise and harder to validate because this 
skips all the parsing and interpreting needed.)

## Version History

### 0.8.0
Hello World -- this is my first attempt at this. The actual numbers
used in the configuration are wrong (they produce some weird outcomes
the farther away you get from about 100 - 150 enrolled in undergrad course),
but for now I just wanted to start the process of systematizing the
calculation. Tuning the numbers can come in the next update. 

This version doesn't do any special-case accounting for specific courses,
such as 0 TA hours allocated for capstone projects.