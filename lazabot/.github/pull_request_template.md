# Pull Request Checklist

## Security Checklist
- [ ] **No secrets or credentials** in code, config files, or commit history
- [ ] **Input validation** implemented for all user inputs
- [ ] **Authentication/authorization** properly implemented where needed
- [ ] **Dependencies** updated and security vulnerabilities addressed
- [ ] **Environment variables** used for sensitive configuration
- [ ] **Error messages** don't expose sensitive information
- [ ] **Logging** doesn't include sensitive data
- [ ] **API endpoints** properly secured and rate-limited
- [ ] **File uploads** validated and sanitized
- [ ] **SQL injection** prevention measures in place

## Testing Checklist
- [ ] **Unit tests** added/updated for new functionality
- [ ] **Integration tests** added/updated for new features
- [ ] **Rust tests** pass: `cargo test --all-features`
- [ ] **Node.js tests** pass: `npm test`
- [ ] **Playwright tests** pass: `node scripts/test_full_integration.js`
- [ ] **Docker build** successful: `docker build -t lazabot:test .`
- [ ] **Manual testing** performed on staging environment
- [ ] **Edge cases** and error scenarios tested
- [ ] **Performance impact** assessed and documented

## Code Quality Checklist
- [ ] **Rust code** follows clippy recommendations
- [ ] **Code formatting** applied: `cargo fmt`
- [ ] **Documentation** updated for new features
- [ ] **README.md** updated if needed
- [ ] **Comments** added for complex logic
- [ ] **Error handling** implemented properly
- [ ] **Logging** added at appropriate levels
- [ ] **Configuration** externalized where appropriate

## Deployment Checklist
- [ ] **Database migrations** included if needed
- [ ] **Configuration changes** documented
- [ ] **Environment variables** documented
- [ ] **Breaking changes** documented and migration path provided
- [ ] **Rollback plan** documented
- [ ] **Monitoring/alerting** updated if needed
- [ ] **Dependencies** compatible with production environment

## Manual Approval Required
- [ ] **Security review** completed by security team
- [ ] **Code review** completed by senior developer
- [ ] **Architecture review** completed if significant changes
- [ ] **Performance review** completed if performance-critical changes
- [ ] **Production deployment** approved by DevOps team

## Pre-deployment Verification
- [ ] **Staging deployment** successful
- [ ] **Smoke tests** pass on staging
- [ ] **Integration tests** pass on staging
- [ ] **Performance benchmarks** meet requirements
- [ ] **Security scan** results reviewed and approved
- [ ] **Backup strategy** verified
- [ ] **Monitoring** configured and tested

## Post-deployment Checklist
- [ ] **Health checks** passing
- [ ] **Metrics** within expected ranges
- [ ] **Error rates** within acceptable limits
- [ ] **Performance** meets SLA requirements
- [ ] **User acceptance** testing completed
- [ ] **Documentation** updated with deployment notes

---

## Additional Notes
Please provide any additional context, concerns, or special considerations for this PR:

## Related Issues
- Closes #(issue number)
- Related to #(issue number)

## Testing Instructions
1. How to test this change locally
2. How to test this change in staging
3. Any special test data or configuration needed

## Rollback Plan
1. Steps to rollback this change if issues arise
2. Any data migration rollback procedures
3. Contact information for rollback approval
