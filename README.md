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

## Usage
The program is a command-line tool. You can run it with `cargo run inputfile.csv` 
where `inputfile.csv` is a CSV file with the course codes and enrollments.

The input file is expected to have the format: 
`Course,Instructor,Enrollment,Lab,Unit Weight`

The instructor column is not currently used for anything and doesn't appear in the
output. It's just in there because it appears in the example docs I got from the
department. I did need to usually manually add the lab column but that's easy to
check in the calendar or schedule of classes.

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

### Following Along
Here's a sample output with made up numbers for a lab course with 1000(!) students.
```
Course ECE459 is considered type UNDERGRAD (unit weight 1.0; lab: yes)
Adding 330.00 hours for Midterm Marking (Calculation Rule: PER_STUDENT)
Adding 670.00 hours for Final Marking (Calculation Rule: PER_STUDENT)
Adding 12.00 hours for Tutorials (Calculation Rule: PER_TERM)
Adding 12.00 hours for Tutorial Prep (Calculation Rule: PER_TERM)
Adding 12.00 hours for Office Hours (Calculation Rule: PER_TERM)
Adding 375.00 hours for Lab Delivery (Calculation Rule: PER_LAB)
Adding 375.00 hours for Lab Prep (Calculation Rule: PER_LAB)
Adding 1100.00 hours for Lab Marking (Calculation Rule: PER_STUDENT)
Adding 5.00 hours for Exam Proctoring (Calculation Rule: PER_TERM)
Adding 0.00 hours for Extra TA Hours (Calculation Rule: PER_TERM)
Total TA hours for ECE459 is calculated at 2891.
This results in a TA allocation of 22.2.
Found special case for course ECE459 of type MAX_ALLOC. Reason: Project Course
Overriding original TA allocation of 22.2 with 6.0
```

## Future Ideas
In no particular order:

* Held-with course support, which is actually two separate cases. There are some
undergrad courses that go under different course codes but should be treated as
one, such as the Software Testing course. Then there are courses that have both 
an undergraduate and graduate section and should get TA support for the combined
enrollment -- but instead of just adding the two together as we might for two
undergrad courses, calculate the numbers for UG and G separately and combine them.

## Version History

### 1.0.0
I guess I can call it 1.0 now? The figures have been updated and I have
some more special case handling for courses with fixed allocation or
per-section allocation. That's probably enough for a v1 and things like
held-with can be in a future revision.

### 0.11.0
Thanks to Simarjeet Saini, I now have more up-to-date values to use in 
the calculator. Those change the results, obviously, but not hte logic.
The only major change is in how the "per lab" items are calculated, which
follows an updated formula based on students per lab and adjusting for LI.
We now expect the lab input to not be yes/no but instead the number of
lab sections the course has.

### 0.10.0
Some courses are lab only, so let's account for that in the calculations.
I also added some more special cases that I learned about.

### 0.9.0
This adds the `SpecialCases` functionality, allowing specific rules for courses
that have unusual allocation needs. The two immediate examples I can think of
are the cleanroom courses that require a minimum number of TAs because of the
logistics of the course and the capstone courses which aren't normally assigned
any TA allocation.

### 0.8.0
Hello World -- this is my first attempt at this. The actual numbers
used in the configuration are wrong (they produce some weird outcomes
the farther away you get from about 100 - 150 enrolled in undergrad course),
but for now I just wanted to start the process of systematizing the
calculation. Tuning the numbers can come in the next update. 

This version doesn't do any special-case accounting for specific courses,
such as 0 TA hours allocated for capstone projects.