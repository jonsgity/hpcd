# Hancock Pattern Program - Pseudocode Description

## Overview
This program analyzes positive integers and classifies them into patterns based on a digit-sum transformation and cycle detection. It visualizes the results as a scatter plot, where each integer is colored by its detected pattern.

## Steps

1. **Parse Command-Line Arguments**
    - Input: base (integer, required)
    - Input: N (integer, optional, default 255)
    - Input: vertical_spacing (integer, optional, default 20)

2. **For each integer n from 1 to N:**
    a. **Cycle Detection**
        - Start with x = n
        - Repeat up to max_iter times:
            - Compute s = sum of digits of x in the given base
            - Set x = s * s
            - If x has been seen before in this sequence:
                - Extract the cycle (the repeated sequence)
                - Normalize the cycle to its minimal rotation (lexicographically smallest rotation)
                - Assign a label to this cycle if not already labeled
                - Break
            - Otherwise, add x to the history
        - If no cycle is found after max_iter, label as 'other'

3. **Labeling**
    - Assign a unique label (a, b, ..., z, aa, ab, ...) to each unique cycle detected
    - Map each integer n to its pattern label

4. **Visualization Preparation**
    - Map each label to a y-value (using vertical_spacing)
    - Assign a unique color to each label
    - Prepare lists of x-values (integers), y-values (pattern positions), and colors

5. **Plotting**
    - Create a scatter plot:
        - x-axis: positive integers (1 to N)
        - y-axis: pattern labels (spaced by vertical_spacing)
        - Each point: colored by its pattern label
    - Add a legend mapping colors to pattern labels
    - Add tooltips or annotations for each point (showing the integer n)
    - Save or display the plot

6. **Pattern Key Output**
    - For each pattern label, print the corresponding cycle as a sequence of numbers (in base representation)

## Helper Functions
- sum_digits(n, base): returns the sum of the digits of n in the given base
- minimal_rotation(cycle): returns the lexicographically smallest rotation of the cycle
- detect_cycle(n, base, max_iter): returns the minimal rotation of the detected cycle for n, or None
- get_label(idx): returns a string label for the idx-th unique cycle
- to_base_digits(n, base): returns a list of digits representing n in the given base

## Notes
- The program is designed to be language-agnostic and can be implemented in Rust, Ruby, Swift, Python, C#, JavaScript, or C++.
- For visualization, use a plotting library appropriate for the language (e.g., matplotlib for Python, plotters for Rust, etc.).
- The cycle detection logic is the core of the program and should be implemented carefully for correctness.
