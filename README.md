# Understanding DOM XSS in Rust with actix-web in Rust 🦀

Show thinking
Of course. A good README.md is essential for any project, especially an educational one. Based on our entire conversation and the final code we developed, here is a comprehensive README file in Markdown format.

Understanding DOM XSS in Rust with actix-web
This project provides a hands-on lesson on understanding, exploiting, and preventing DOM-based Cross-Site Scripting (XSS) vulnerabilities within a Rust web application using the actix-web framework. It is based on the Snyk Learn lesson on DOM XSS, with all concepts and code adapted for a Rust backend context.

The primary goal is to demonstrate how a secure language like Rust can still serve client-side JavaScript that introduces security risks, and how to apply modern Rust practices to mitigate these risks effectively.

🎯 Learning Objectives
After exploring this project, you will understand:

What a DOM-based XSS vulnerability is and how it differs from reflected or stored XSS.
How a Rust actix-web backend can serve pages that are vulnerable on the client-side.
The critical difference between dangerous JavaScript "sinks" like innerHTML and safe sinks like .textContent.
Multiple layers of defense against XSS, including:
Choosing safe sinks in client-side code.
Performing server-side sanitization of user input with crates like ammonia.

Implementing a Content Security Policy (CSP) as a powerful HTTP header defense.

How to craft and test XSS payloads to confirm a vulnerability.
⚙️ Prerequisites
To run this project, you will need to have Rust and Cargo installed. You can install them from the official Rust website: https://www.rust-lang.org/tools/install

🚀 Getting Started
Clone the repository:

git clone https://github.com/olsigjecii/dom_xss
cd dom_xss
Run the application:

```bash
cargo run
```

# You will see the server start:

Server running at http://127.0.0.1:3000
- Working Vulnerable Color Page: http://127.0.0.1:3000/vulnerable_color
- Final Secure Color Page:     http://127.0.0.1:3000/secure_color
... (and other routes)

🔬 Project Endpoints & Demonstrations
The project exposes several endpoints to demonstrate different concepts.

1. The Core Vulnerability (innerHTML)
This example shows how using .innerHTML can lead to a DOM XSS vulnerability.

Vulnerable Endpoint: http://127.0.0.1:3000/vulnerable_color
Description: This page takes a color parameter and uses the vulnerable innerHTML property to display it on the page.
How to Test (Exploit): Visit the following URL. An alert box should appear, confirming the script execution.
http://127.0.0.1:3000/vulnerable_color?color=%3Cimg%20src=x%20onerror=%22alert('XSS%20via%20innerHTML!')%22%3E
2. Mitigation 1: Using Safe Sinks
This is the primary and most effective fix: writing client-side code that is inherently safe.

Secure Endpoint: http://127.0.0.1:3000/secure_color
Description: This page uses the safe .textContent property. .textContent treats all input as plain text, preventing the browser from interpreting it as HTML.
How to Test: Use the same payload as before. You will see the malicious string printed harmlessly on the page, with no alert box.
http://127.0.0.1:3000/secure_color_example?color=%3Cimg%20src=x%20onerror=%22alert('This%20will%20not%20fire')%22%3E
3. Mitigation 2: Server-Side Sanitization
This approach demonstrates cleaning user input on the server before it's even sent to the browser.

Sanitized Endpoint: http://127.0.0.1:3000/sanitized_color
Description: The Rust handler for this route uses the ammonia crate to strip all HTML tags from the color parameter before embedding it in the page.

How to Test: Using the malicious payload will result in the `<img>` tag being completely removed by the server. The style will be applied with an empty string.
http://127.0.0.1:3000/sanitized_color?color=%3Cimg%20src=x%20onerror=%22alert('This%20will%20not%20fire')%22%3E
4. Mitigation 3: Content Security Policy (CSP)
This shows how the Rust server can instruct the browser to enforce a security policy as a powerful second layer of defense.

CSP Endpoint: http://127.0.0.1:3000/csp
Description: The actix-web handler adds a Content-Security-Policy HTTP header to the response. This policy blocks all inline scripts and event handlers, even if the page had a vulnerability.


How to Test: You can view the response headers in your browser's developer tools (Network tab) to see the Content-Security-Policy header in action.
🔑 Key Concepts Summary
Dangerous Sinks: Functions or properties that cause the browser to interpret input as executable code. In this project, we demonstrated document.write and innerHTML.
Safe Sinks: Properties that are designed to handle data as plain text or in a specific, safe context. We used .textContent and .style.color as examples.
Defense in Depth: Relying on a single line of defense is risky. A secure application uses multiple layers, such as safe client-side code, server-side sanitization, and browser-level policies like CSP.

```bash

# Test the Vulnerable Route:
http://127.0.0.1:3000/vulnerable_color?color=%3Cimg%20src=x%20onerror=%22alert(%27It%20Worked!%27)%22%3E
# -----------------------------------------------------------------------------------------

# Test the Secure Route:
http://127.0.0.1:3000/secure_color?color=%3Cimg%20src=x%20onerror=%22alert(%27It%20Worked!%27)%22%3E

# Test the Secure Route with amonia sanitization crate:
http://127.0.0.1:3000/sanitized_color?color=%3Cimg%20src=x%20onerror=%22alert(%27It%20Worked!%27)%22%3E

# Test the Secure Route with CSP headers:
http://127.0.0.1:3000/csp?color=%3Cimg%20src=x%20onerror=%22alert(%27It%20Worked!%27)%22%3E
```
