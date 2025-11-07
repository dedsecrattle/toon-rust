# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in `toon-rust`, please report it responsibly.

### How to Report

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email the maintainer directly at: **itsprabxxx@gmail.com**

Include the following information:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

### What to Expect

- You will receive an acknowledgment within 48 hours
- We will investigate and provide an initial assessment within 7 days
- We will keep you informed of our progress
- Once fixed, we will credit you in the security advisory (unless you prefer to remain anonymous)

### Disclosure Policy

- We will disclose the vulnerability after a fix is available
- We will coordinate with you on the disclosure timeline
- Security fixes will be released as patch versions when possible

## Security Best Practices

When using `toon-rust`:

1. **Always validate input** - Don't trust untrusted TOON data
2. **Use strict mode** - Enable strict validation in `DecodeOptions` when parsing untrusted input
3. **Keep dependencies updated** - Run `cargo update` regularly
4. **Review changes** - Review dependency updates for security implications

## Known Security Considerations

- **Parsing untrusted input**: Always use strict validation mode
- **Memory limits**: Very large inputs may cause memory issues
- **DoS potential**: Malformed input could cause excessive CPU usage

Thank you for helping keep `toon-rust` secure!

