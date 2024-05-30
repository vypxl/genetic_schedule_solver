// Default background color
#let default_bg = rgb("E2E9E3")

// Default text layout
#set text(fill: rgb("333936"), font: "Avenir", size: 18pt)

#let today = datetime(year: 2024, month: 5, day: 30)

// Footer text

#let footer_text = [Using a Genetic Algorithm to Solve Class Scheduling - Thomas Meyer-Lehnert]

// SLIDES

// The default slide
#let slide(bg: default_bg, has_footer: true, content) = {
  show heading.where(level: 1): hd => [
    #set text(size: 26pt)
    #place(top)[#underline(stroke: 2pt, offset: 3pt, hd.body)]
  ]
  page(paper: "presentation-16-9", footer-descent: 40%, footer: if has_footer [
      #set text(10pt, fill: rgb("3e503c"))
    #place(center, footer_text)
    #set align(end)
      #counter(page).display(
        "1 / 1",
        both: true,
      )
    ], fill: bg, margin: (x: 12%, y: 10%))[
  #v(10%)
  #align(horizon)[#content]
  ]
}

// The title slide with an optional subtitle
#let titleSlide(title, subtitle: "", author: "", date: today.display("[day].[month].[year]")) = {
  slide(has_footer: false)[#align(center + horizon, [
    #text(32pt)[* #title * ]\
    \
    #if subtitle != "" [#text(26pt)[#subtitle]\ \ ]
    #if author != "" [#text(22pt)[#emph(author)]\ \ ]
    #text(18pt)[#date]
    ])
  ]
}

#let citef(it) = footnote(cite(it, form: "author"))


#titleSlide(subtitle: "Optimization Methods for Engineers - Project Presentation", author: [Thomas Meyer-Lehnert (`21-949-292`)])[Using a Genetic Algorithm to Solve Class Scheduling]

#slide[
  = Overview

  - Problem Description
  - Genetic Algorithm
  - Results
  - Lessons Learned
  - Conclusion
]

#slide[
  = Problem Description - Motivation

  Schools and universities have to create a timetable for their classes.

  This timetable has requirements, e.g.:
  - It has to include all classes to be taught
  - It must not have one professor teaching two classes at the same time
  - It should not have overlaps of classes for students
  - < ... >

  -> Find the 'best' timetable according to some evalutation criteria

  - Generally NP-hard #citef(<aome>) #citef(<abuay>)
  - Simplified version of the 'University Course Timetabling Problem' #citef(<abuay>)
]

#slide[
  = Problem Description - Overview

  We want find timetables that that assign each timeslot (e.g. Monday 10-12) and room to a class (or no class).
  We then want to optimize this to find a timetable that is 'best', so for example minimize class overlaps for students.

  The model will be simplfied: Classes, Rooms, Timeslots are integers, Students and Professors sets of integers (the classes they take/teach).

  Each class just has one lesson, each room has infinite capacity.

  \
  #box(fill: silver, inset: 5pt)[One could define a lot more constaints, but to showcase the genetic algorithmic approach, this is sufficient. More mandatory constraints and optimization criteria could easily be added.]
]

#slide[
  = Problem Description - Formal

  - We have a set of Constraints \ #h(.5cm) $CC = {C = "Classes", S = "Students", P = "Professors", R = "Rooms", T = "Timeslots"}$
  - $C subset NN$, $R subset NN$, $T subset NN$
  - Each 'student' $s in S$ & each 'professor' $p in P$ is a set of classes $C_s subset.eq C$ (resp. $C_p$)

  A Timetable $Q in QQ$ is a bijective function $T times R -> C$ that satisfies:

  - All classes are held: $forall c in C exists t in T, r in R: Q(t, r) = c$
  - No professor teaches two classes at the same time: \ #h(.5cm) $forall p in P, t in T: |{c in C_p: Q(t, r) = c}| <= 1$

  As a bijective function, $Q$ has an inverse $Q^(-1): C -> T times R$

  We want to find a timetable $Q$ that minimizes some evaluation function $f: QQ -> RR^+$
]

#slide[
  = Problem Description - Evaluation Function

  We define the evaluation function $f(Q)$ as follows:

  $f(Q) &= sum_(s in S) \
    & (|{c in C_s | exists c' in C_s "such that c' is held at the same time as c"}| - 1) \
    & + (|{t in T | Q("_", t-1) in C_s and Q("_", t+1) in C_s and Q("_", t) in.not C_s}|)
  $

  This function counts the number of overlaps of classes for students and the number of 'gaps' in the timetable.
  The second criterium was chosen to make the algorithm prefer 'blocks' of classes over fragmented schedules.

  A more sophisticated evaluation function could include lunch breaks, prefer morning classes, prefer one full day over many sparsely filled days, etc.
]

#slide[
  = Genetic Algorithm - Overview

  In general, a genetic algorithm can be used to optimize a problem in the following way:

  1. Represent a possible solution to the Problem as a 'Genome'
  2. Create a population of Genomes
  3. Evaluate each Genome using an evaluation function
  4. Select the best Genomes from the population
  5. Create a new population by 'breeding' (combining/crossing) the best genomes and mutate them with a small probability
  6. Repeat from step 3 until a stopping criterion is met
]

#slide[
  = Genetic Algorithm - Application

  In our case, a genome is a timetable $Q$. We represent it as a table of size $|T| times |R|$ with entries in $C$.
  The special value of $0$ means 'no class'.

  Using the steps outline before, we can iteratively find timetables minimizing the evaluation function $f$.

  We now need to define a combining function (from now on called `cross`) and a mutation function `mutate`.
]

#slide[
  = Genetic Algorithm - Combining Function `cross`
  `cross` takes two timetables $Q_l, Q_r$ and creates a new timetable $Q^*$.

  We define it as follows:
  1. Begin with a timetable filled with zeros for no class.
  2. For each timeslot $t$ and room $r$, we choose $Q^*(t, r) = Q_l(t, r)$ with probability $0.5$, otherwise $Q_r(t, r)$. If this would create a duplicate assignment (class taught in two slots), we skip it.

  Now we repair possible constraint violations:

  3. For each class, if it is not held, we add it to a random free timeslot and room
]

#slide[
  = Genetic Algorithm - Mutation Function `mutate`

  `mutate` takes a timetable $Q$ and returns a new timetable $Q^*$.

  We define it as follows:

  For each timeslot and room:

  1. Mutate with probability $rho$
  2. Remove this class from the slot
  3. Place it into a random other free slot
]

#slide[
  = Genetic Algorithm - Selection & Iteration

  We select the best timetables from the population using the evaluation function $f$.
  We sort the population by $f$ and take the best $k$ timetables.

  Then we create a new population by making $k/2$ pairs and crossing them, then mutating the 'children' with probability $rho$.

  We repeat this for the desired number of generations to obtain our final population.
  The result will then be the timetable from this population with the lowest evaluation score.
]

#slide[
  = Genetic Algorithm - What about the Professor Constraint?

  We did not fix the possible violation of the constraint that no professor teaches two classes at the same time.

  We could handle this similarly to the other constraints, but I found that in this case, a different approach leads to better convergence behavior:

  Instead of trying to fix non-compliant timetables, we just add a strong penalty to the evaluation function.

  This way, the population will gradually evolve to not violate this constraint, as it will always be a large disadvantage.
]

#slide[
  = Genetic Algorithm - Implementation

  I implemented the described algorithm in Rust #footnote[See appendix for relevant code]. It takes the number of courses, rooms and timeslots ($|C| <= |T| * |R|$ !), and definitions for each student and professor. It can also generate mock data for testing.

  For a more detailed description of the input format and how to use the program, consult the `README.md` file.


]

#slide[
  = Results

  For demonstration purposes, I created three test sets: 

  > Note: Each Course is taught by one professor, so the number of professors is determined by `Cs/Prof`

  > Note 2: Due to the random nature of the generated data, results will be much less optimal than in a real life scenario.

  #table(
    columns: 7,
    inset: 10pt,
    align: center,
    table.header(
      [*Test Set*], [*Students*], [*Courses*], [*Timeslots*], [*Rooms*], [*Cs/Stud*], [*Cs/Prof*]
    )
    ,
    "Small", "100", "16", "10", "3", "3", "2",
    "Medium", "256", "50", "20", "5", "8", "5",
    "Realistic", "5000", "100", "40", "10", "20", "10"
  )
]

//
// image("results/all-1024-64-0.02-0.0025.png"),
// image("results/all-1024-64-0.02-0.005.png"),
// image("results/all-1024-64-0.02-0.02.png"),
// image("results/all-1024-64-0.02-0.05.png"),
//
// image("results/all-1024-64-0.2-0.02.png"),
//
// image("results/all-1024-64-0.05-0.02.png"),
//
// image("results/all-128-128-0.05-0.02.png"),
// image("results/all-4096-64-0.05-0.02.png"),

#slide[
  = Results - Varying Population Size

  We see populations sizes 128, 1024, and 4096
  #table(
    columns: 3,
    image("results/all-128-128-0.05-0.02.png"),
    image("results/all-1024-64-0.05-0.02.png"),
    image("results/all-4096-64-0.05-0.02.png"),
  )
]

#slide[
  = Results - Varying Selection Split

  We see Selection Splits of 2%, 5%, and 20%

  Note that at 20% split, the algorithm could not generate a valid timetable within 64 Generations for the realistic example.

  #table(
    columns: 3,
    image("results/all-1024-64-0.02-0.02.png"),
    image("results/all-1024-64-0.05-0.02.png"),
    image("results/all-1024-64-0.2-0.02.png"),
  )
]

#slide[
  = Results = Varying Mutation Chance

  We see Mutation Chances of 0.25%, 0.5%, 2%, 5%
  This means each newly generated timetable has a X% chance per slot to mutate in that slot.

  #table(
    columns: 4,
    image("results/all-1024-64-0.02-0.0025.png"),
    image("results/all-1024-64-0.02-0.005.png"),
    image("results/all-1024-64-0.05-0.02.png"),
    image("results/all-1024-64-0.02-0.05.png"),
  )
]

#slide[
  = Lessons Learned

  - The Meta-parameters greatly influence effectiveness
  - Same goes for problem modeling
  - Not always obvious why a change leads to better results
  - Implementation details can be tricky: Bugs, oversights, ..

  - Pro: Very well parallelizable: almost linear speedup with more cpu cores! (Evaluations of genomes is completely independent, and make up most of the computation)

  -> Genetic Algorithms are a powerful tool, but require a lot of experimentation and tuning
]

#slide[
  = Conclusion

  Genetic Algorithms are a good tool to approach optimization problems that benefit from stochastic search. For small to medium sized problems, they can be very fast and effective. For larger problems, a lot of fine tuning is needed to achieve good results. However, by taking inspiration from nature, simple functions for mutation and combining yield impressive results.
]

#slide(bibliography("bibliography.yml", title: "Sources"))
