# Yuki CLI

⚠️ 注意：项目已经移到[这里](https://github.com/ustclug/Yuki/blob/master/cmd/yukictl/README.md)，该仓库已废弃。

### Table of Content

* [Introduction](#introduction)
* [Tips](#tips)

### Introduction

[Yuki(ustcsync)](https://github.com/ustclug/Yuki) 的命令行客户端.

### Tips

未介绍到的功能可以执行 `yuki SUBCOMMAND -h` 查看.

#### 自动补全

```
# Zsh:
yuki completion zsh

# Bash:
yuki completion bash
```

#### 增加新的配置

把以下内容保存为 `repo.json` :
```
[
    {
      "name": "brew.git",  // 必需
      "image": "ustcmirror/test:latest",  // 必需
      "interval": "53 * * * *",  // 必需 (crontab)
      "storageDir": "/path/to/dir",  // 必需

      "bindIP": "1.2.3.4",
      "envs": {
        "key": "val",
      },
      "logRotCycle": 10,
      "retry": 3,
      "user": "1002:1002",
      "volumes": {
          "/host/dir": "/container/dir"  // 注意: 受 MongoDB 限制, host dir 路径中不能含有 '.'
      }
    }
]
```

然后执行 `yuki import repo.json`.

另外也可以导出现有的配置, 然后再修改:
```
yuki export --pretty $repo > repo.json
vim repo.json
yuki import repo.json
```

#### 更新现有的配置

增加/更新某些键值:
```
yuki repo update archlinux envs.RSYNC_HOST=example.com envs.RSYNC_PATH=mod bindIP=1.2.3.4
```

删除某些键值:
```
yuki repo update archlinux envs.RSYNC_EXTRA=
```

#### 获取同步日志

当前的同步日志:
```
yuki ct logs <container ID or repo name>
```

跟踪并从倒数第 5 行开始:
```
yuki ct logs --tail 5 -f <container ID or repo name>
```

列出以往的同步日志:
```
yuki repo logs --stats <repo name>
```

查看以往倒数第二次同步日志的倒数 10 行:
```
yuki repo logs -n 1 --tail 10 <repo name>
```
