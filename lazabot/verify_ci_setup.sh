#!/bin/bash

echo "🔍 Verifying CI/CD Setup..."

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Not in the correct directory. Please run from the project root.${NC}"
    exit 1
fi

echo -e "${YELLOW}📁 Checking required files...${NC}"

# Check GitHub Actions workflow
if [ -f ".github/workflows/ci.yml" ]; then
    echo -e "${GREEN}✅ Main CI workflow exists${NC}"
else
    echo -e "${RED}❌ Main CI workflow missing${NC}"
fi

# Check PR template
if [ -f ".github/pull_request_template.md" ]; then
    echo -e "${GREEN}✅ PR template exists${NC}"
else
    echo -e "${RED}❌ PR template missing${NC}"
fi

# Check security policy
if [ -f ".github/SECURITY.md" ]; then
    echo -e "${GREEN}✅ Security policy exists${NC}"
else
    echo -e "${RED}❌ Security policy missing${NC}"
fi

# Check ESLint config
if [ -f ".eslintrc.js" ]; then
    echo -e "${GREEN}✅ ESLint configuration exists${NC}"
else
    echo -e "${RED}❌ ESLint configuration missing${NC}"
fi

# Check test script
if [ -f "scripts/run_all_tests.sh" ]; then
    if [ -x "scripts/run_all_tests.sh" ]; then
        echo -e "${GREEN}✅ Test script exists and is executable${NC}"
    else
        echo -e "${YELLOW}⚠️  Test script exists but not executable${NC}"
        chmod +x scripts/run_all_tests.sh
        echo -e "${GREEN}✅ Made test script executable${NC}"
    fi
else
    echo -e "${RED}❌ Test script missing${NC}"
fi

# Check Dependabot config
if [ -f ".github/dependabot.yml" ]; then
    echo -e "${GREEN}✅ Dependabot configuration exists${NC}"
else
    echo -e "${RED}❌ Dependabot configuration missing${NC}"
fi

# Check issue templates
if [ -f ".github/ISSUE_TEMPLATE/bug_report.md" ] && [ -f ".github/ISSUE_TEMPLATE/feature_request.md" ]; then
    echo -e "${GREEN}✅ Issue templates exist${NC}"
else
    echo -e "${RED}❌ Issue templates missing${NC}"
fi

# Validate YAML syntax
echo -e "${YELLOW}🔧 Validating YAML syntax...${NC}"
if command -v python3 >/dev/null 2>&1; then
    python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))" 2>/dev/null && echo -e "${GREEN}✅ CI workflow YAML is valid${NC}" || echo -e "${RED}❌ CI workflow YAML is invalid${NC}"
    python3 -c "import yaml; yaml.safe_load(open('.github/dependabot.yml'))" 2>/dev/null && echo -e "${GREEN}✅ Dependabot YAML is valid${NC}" || echo -e "${RED}❌ Dependabot YAML is invalid${NC}"
else
    echo -e "${YELLOW}⚠️  Python3 not available, skipping YAML validation${NC}"
fi

# Check file permissions
echo -e "${YELLOW}🔐 Checking file permissions...${NC}"
ls -la .github/workflows/ci.yml | grep -q "^-rw" && echo -e "${GREEN}✅ CI workflow has correct permissions${NC}" || echo -e "${YELLOW}⚠️  CI workflow permissions may need adjustment${NC}"

# Summary
echo -e "\n${YELLOW}📊 Setup Summary:${NC}"
echo "Files created: $(find .github -type f | wc -l) GitHub configuration files"
echo "Test script: $(test -x scripts/run_all_tests.sh && echo "Ready" || echo "Needs permissions")"
echo "Documentation: $(test -f README.md && echo "Updated" || echo "Missing")"

echo -e "\n${GREEN}🎉 CI/CD Setup Verification Complete!${NC}"
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Push changes to GitHub"
echo "2. Create a test pull request"
echo "3. Verify GitHub Actions run successfully"
echo "4. Test the deployment process"
