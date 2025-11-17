## 实用脚本

### `pre-push` 脚本：检查变更中是否有禁止的内容

使用 `deno run -A pre-push.ts` 来执行此脚本。此脚本会检查即将推送的变更中是否包含禁止的内容（如 `TODO`、`FIXME`、`console.log` 等）。如果发现这些内容，脚本会输出错误信息并阻止推送。

```ts
import $ from 'jsr:@david/dax'

// 定义需要检查的模式
const forbiddenPatterns = [
  { regex: /TODO|FIXME|DEBUG|WIP/i, message: 'TODO/FIXME/DEBUG/WIP' },
  { regex: /console\.log/i, message: 'console.log' },
]

async function checkDiff() {
  try {
    // 获取 git 格式的 diff
    const diffOutput = await $`hj diff -r "remote_bookmarks(remote=origin)..bookmarks()" --git`.text()

    const lines = diffOutput.split('\n')
    const errors: string[] = []

    // 遍历每一行
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i]

      // 只检查新增的行（以 + 开头，但不是 +++ 文件头）
      if (line.startsWith('+') && !line.startsWith('+++')) {
        // 检查每个禁止的模式
        for (const pattern of forbiddenPatterns) {
          if (pattern.regex.test(line)) {
            errors.push(`Line ${i + 1}: Found ${pattern.message}`)
            errors.push(`  ${line}`)
          }
        }
      }
    }

    // 如果发现错误，输出并以错误码退出
    if (errors.length > 0) {
      console.error('\n❌ Pre-commit check failed! Found forbidden patterns:\n')
      errors.forEach(error => console.error(error))
      console.error('\nPlease remove these patterns before committing.\n')
      Deno.exit(1)
    }

    console.log('✅ Pre-commit check passed!')
    Deno.exit(0)

  } catch (error) {
    console.error('Error running diff command:', error)
    Deno.exit(1)
  }
}

await checkDiff()
```
