# Agents Code Standards

## Language Requirements

All code in this project MUST be written in English, with the following exceptions:

### Allowed Non-English Content
- **Language files**: Localization/i18n files (e.g., `.json`, `.yaml`, `.properties` files containing translations)
- **User-facing strings**: When specifically targeting non-English speaking users
- **Comments**: Documentation comments may include non-English text when explaining complex concepts for non-English speaking developers

### Required English Content
- **Variable names**: All variables, functions, classes, and constants
- **Function names**: All method and function declarations
- **File names**: All source code files (except language files)
- **Directory names**: All folder structures
- **Comments**: General code comments should be in English
- **Documentation**: README files, API documentation, and technical documentation
- **Error messages**: Error messages and logs
- **Configuration**: Configuration files and settings

## Examples

### ✅ Correct (English)
```javascript
const userService = new UserService();
const maxRetries = 3;

function getUserData(userId) {
  return userService.findById(userId);
}
```

### ❌ Incorrect (Non-English)
```javascript
const 用户服务 = new UserService();
const 最大重试次数 = 3;

function 获取用户数据(用户ID) {
  return userService.findById(用户ID);
}
```

### ✅ Allowed (Language File)
```json
{
  "welcome": "欢迎",
  "goodbye": "再见",
  "error_message": "发生错误"
}
```

## Enforcement

This standard will be enforced through:
1. Code review processes
2. Linting rules where applicable
3. Automated checks in CI/CD pipelines

## Purpose

This standard ensures:
- Code maintainability across international teams
- Consistent codebase language
- Better collaboration and knowledge sharing
- Easier onboarding for new developers

## Exceptions

Exceptions to this rule must be:
1. Documented with clear reasoning
2. Approved by the project maintainers
3. Limited in scope and necessity

---

*Last updated: April 2026*
