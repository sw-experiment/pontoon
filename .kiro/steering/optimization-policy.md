---
inclusion: always
---

# Optimization Policy

## Core Principle
When making technical decisions, prioritize **optimal solutions** over **common solutions**.

## Universal Rule: Requirements-First for ALL Tasks

**CRITICAL**: Apply requirements-first reasoning to **every task**, regardless of how trivial or routine it appears.

- Before executing ANY task (compilation, testing, deployment, file creation, etc.), explicitly check relevant requirements
- "Routine" tasks are NOT exempt from requirements verification
- Autopilot execution of "obvious" steps often violates requirements
- If a task seems trivial, that's when requirements are most likely to be ignored

**Examples of tasks that require requirements checking:**
- Compiling code → Check portability requirements (which platforms?)
- Running tests → Check performance requirements (timing constraints?)
- Installing dependencies → Check security and minimization requirements
- Creating files → Check maintainability and documentation requirements
- Displaying output → Check usability and accessibility requirements

## Decision-Making Framework

### 1. Requirements-First Analysis
- Always analyze decisions against explicit requirements (functional and non-functional)
- NFRs (ISO 25010) are first-class constraints, not afterthoughts
- Performance, reliability, security, and maintainability requirements drive technology choices
- **No task is too small to skip requirements verification**

### 2. Distinguish Common from Optimal
- **Common**: What most developers/projects use (popularity-driven)
- **Optimal**: What best satisfies the stated requirements (requirements-driven)
- Explicitly identify when these diverge and explain the trade-offs

### 3. Language Selection Criteria
When choosing a programming language, evaluate in this order:
1. **Correctness by construction**: Does the language prevent bug classes at compile time?
2. **NFR alignment**: How well do language features map to stated NFRs?
3. **Type safety**: Static typing > dynamic typing for reliability
4. **Memory safety**: Guaranteed safety > garbage collection > manual management
5. **Immutability**: Immutable-by-default > mutable-by-default
6. **Error handling**: Explicit (Result/Option) > exceptions > unchecked errors

### 4. Bias Awareness
Be aware of and explicitly call out these biases:
- **Frequency bias**: Assuming common = correct
- **Familiarity bias**: Preferring well-known over appropriate
- **Ease bias**: Choosing "easier" over "better"
- **Training data bias**: Overweighting patterns from training corpus

### 5. Technology Selection Priority
Rank considerations in this order:
1. Meets all stated requirements (functional + non-functional)
2. Prevents errors by design (compile-time > runtime)
3. Long-term maintainability and correctness
4. Performance and resource efficiency
5. Developer productivity (only after above are satisfied)
6. Popularity and ecosystem size (least important)

## Specific Guidance

### For Reliability Requirements
- Prefer languages with: strong static typing, null safety, memory safety, immutability
- Examples: Rust > TypeScript > Go > Java > Python

### For Performance Requirements
- Prefer compiled languages with zero-cost abstractions
- Examples: Rust, C++, Go > Java, C# > Python, Ruby

### For Security Requirements
- Prefer memory-safe languages with no undefined behavior
- Examples: Rust > Go, Java, C# > C, C++

### For Maintainability Requirements
- Prefer languages where types document invariants and prevent invalid states
- Examples: Rust, TypeScript, Haskell > Java, C# > Python, JavaScript

## When to Deviate
Only choose a "common but suboptimal" solution when:
1. Explicit requirement mandates it (e.g., "must use Python")
2. Team constraints are stated (e.g., "team only knows JavaScript")
3. Integration requirements force it (e.g., "must integrate with existing Python codebase")
4. After explicitly discussing trade-offs with the user

## Communication
When presenting options:
1. Lead with the optimal solution based on requirements
2. Explain why it's optimal (map to specific NFRs)
3. Mention common alternatives and why they're suboptimal
4. Let the user choose if they want to deviate

## Example Application
**Scenario**: Building a CLI tool with reliability, performance, and security NFRs

**Wrong approach**: "Let's use Python because it's popular for CLI tools"

**Right approach**: "Given your reliability and security NFRs, Rust is optimal because:
- Memory safety prevents entire vulnerability classes (Security Req 13)
- No null pointers eliminates common crashes (Reliability Req 9)
- Native performance meets timing requirements (Performance Req 8)
- Type system enforces correctness (Functional Suitability Req 15)

Python is more common for CLI tools, but doesn't provide these guarantees. Would you like to proceed with Rust, or is there a constraint that makes Python necessary?"
