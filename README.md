# RustSL

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![GUI: PyQt5](https://img.shields.io/badge/GUI-PyQt5-blue.svg)](https://www.riverbankcomputing.com/software/pyqt/)

一款基于 Rust 开发的模块化免杀框架，支持 GUI 可视化与灵活的二次开发，内置多种 Shellcode 伪装与反沙箱策略。

![alt text](static/front.png)

## ✨ 特性

### 🔐 加密方式
- **RC4** - 流加密算法
- **IPv4** - 将 Shellcode 转换为 IPv4 地址格式
- **IPv6** - 将 Shellcode 转换为 IPv6 地址格式
- **MAC** - 将 Shellcode 转换为 MAC 地址格式
- **UUID** - 将 Shellcode 转换为 UUID 格式
- 可拓展...

### 💾 内存分配
- **VirtualAlloc** - 使用系统 API 分配 RWX 内存
- 可拓展...

### 🛡️ VM/沙箱检测
- **Tick 检测** - 时间差异分析
- **鼠标轨迹检测** - 通过多点轨迹特征判断真实鼠标活动
- 可拓展...

### 🚀 运行模式
- **CreateThread 直接执行** - 传统线程创建方式
- 可拓展...

## 📦 项目结构

```
RSL/
├── gui/                     # PyQt5 图形界面与组件
│   ├── main_window.py       # 主窗口逻辑
│   ├── widgets.py           # 自定义控件
│   ├── sign.py              # 签名相关界面与逻辑
│   ├── config_manager.py    # 配置管理
│   ├── styles.py            # 样式表与主题
│   ├── worker.py            # 后台任务与多线程
│   ├── ui_components.py     # 复用 UI 组件
│   ├── __init__.py          # 包初始化
│   └── icons/               # 内部图标资源
├── src/                     # Rust 核心代码
│   ├── main.rs              # Rust 主程序入口
│   ├── alloc_mem/           # 内存分配相关模块
│   ├── decrypt/             # Shellcode 解密模块
│   ├── exec/                # Shellcode 执行模块
│   ├── forgery/             # 资源伪造与混淆
│   ├── guard/               # 反沙箱/反虚拟机检测
│   └── utils/               # 工具函数
├── config/                  # 配置文件目录
│   └── plugins.json         # 插件与功能配置
├── encrypt_plugins/         # Python 加密插件目录（每个插件为独立模块）
│   ├── __init__.py
│   ├── ipv4.py
│   ├── ipv6.py
│   ├── mac.py
│   ├── uuid.py
│   └── rc4.py
├── sign/                    # 签名相关与第三方工具
│   ├── sigthief.py          # 签名伪造脚本
│   └── app/                 # 第三方签名工具存放
├── encrypt.py               # Shellcode 加密脚本
├── main.py                  # GUI 启动入口
├── Cargo.toml               # Rust 项目配置文件
├── build.rs                 # Rust 构建脚本
├── requirements.txt         # Python 依赖列表
├── input/                   # Shellcode 输入目录
├── output/                  # 生成的可执行文件输出目录
├── static/                  # 静态资源（如图片、截图）
├── icons/                   # 额外图标资源
└── rust-toolchain.toml      # Rust 工具链配置
```

## 🚀 快速开始

### 环境要求

- **Python 3.7+**
- **Rust Nightly** (需配合 rustup 使用)
- **PyQt5**
- **Cargo** (Rust 包管理器)

### 安装依赖
#### Python 依赖
```bash
pip install -r requirements.txt
```

**依赖说明：**
- `PyQt5` - 图形界面框架
- `pycryptodome` - 加密库（ChaCha20, AES-GCM, RC4 等）

#### Rust 环境配置

本项目依赖 Rust Nightly 版本及 `build-std` 特性以优化体积和去除特征。

1. **安装 Rustup**
   - Windows: 下载并运行 [rustup-init.exe](https://win.rustup.rs/)

2. **配置 Nightly 工具链**
   项目根目录已包含 `rust-toolchain.toml`，进入目录后 Rustup 会自动检测。你需要手动安装 Nightly 工具链及源码组件：
   ```bash
   # 安装 nightly 工具链
   rustup install nightly

   # 安装 rust-src 组件（用于 build-std 重新编译标准库）
   rustup component add rust-src --toolchain nightly
   
   # 添加 Windows MSVC 目标（通常默认已安装）
   rustup target add x86_64-pc-windows-msvc --toolchain nightly
   ```

3. **验证环境**
   ```bash
   cargo +nightly --version
   ```

### 使用方法

#### 1. 启动 GUI
```bash
python main.py
```

#### 2. 选择配置
在 GUI 界面中选择：
- **加密方式**：RC4 / IPv4 / IPv6 / MAC / UUID
- **运行模式**：CreateThread
- **内存分配**：VirtualAlloc
- **VM 检测**：勾选需要的检测项

#### 3. 生成加载器
点击 **"一键生成"** 按钮，程序将自动完成：
- Shellcode 加密
- Rust 编译（带特性选择）
- 文件复制到 `output/` 目录
- 签名伪造（如启用）

## 🔒 免杀效果

本项目通过多种技术手段实现对安全软件的检测绕过：

- **加密保护**：支持 RC4、IPv4、IPv6、MAC、UUID 等多种 Shellcode 编码/加密方式，防止静态分析工具直接识别恶意代码
- **环境检测**：集成 Tick 计数检测、鼠标轨迹检测和桌面文件数量检测，能够识别虚拟机或沙箱环境，避免在可疑环境中执行
- **执行方式**：采用 CreateThread 直接执行模式，通过动态 API 解析减少导入表特征，降低被检测的风险
- **代码混淆**：Rust 编译器的优化和无默认特征编译，进一步减小可执行文件的特征指纹

这些特性组合使用，能够有效提高 Shellcode 加载器的隐蔽性和生存能力。

## ⚙️ 配置文件

`config/plugins.json` 控制所有功能模块：

```json
{
  "encryption": [
    { "id": "ipv4", "label": "ipv4", "encrypt_arg": "ipv4", "feature": "decrypt_ipv4" },
    { "id": "ipv6", "label": "ipv6", "encrypt_arg": "ipv6", "feature": "decrypt_ipv6" },
    { "id": "mac", "label": "mac", "encrypt_arg": "mac", "feature": "decrypt_mac" },
    { "id": "uuid", "label": "uuid", "encrypt_arg": "uuid", "feature": "decrypt_uuid" },
    { "id": "rc4", "label": "rc4", "encrypt_arg": "rc4", "feature": "decrypt_rc4" }
  ],
  "alloc_mem_modes": [
    { "id": "alloc_mem_va", "label": "VirtualAlloc", "feature": "alloc_mem_va" }
  ],
  "run_modes": [
    { "id": "create_thread", "label": "CreateThread 直接执行 (create_thread)", "feature": "run_create_thread" }
  ],
  "vm_checks": [
    { "id": "tick", "label": "Tick检测", "feature": "vm_check_tick" },
    { "id": "mouse", "label": "鼠标轨迹", "feature": "vm_check_mouse" },
    { "id": "desktop_files", "label": "桌面文件", "feature": "vm_check_desktop_files" }
  ],
  "defaults": {
    "encryption": "ipv4",
    "run_mode": "create_thread",
    "alloc_mem_mode": "alloc_mem_va"
  }
}
```

## 🔧 命令行加密

也可以单独使用加密脚本：

```bash
python encrypt.py -i input.bin -o output.bin -m ipv4
```

参数：
- `-i, --input` - 输入的二进制文件
- `-o, --output` - 输出的加密文件
- `-m, --method` - 加密方式 / 插件名称（列出 `encrypt_plugins/` 目录下的插件名）

说明：当前 `encrypt.py` 使用插件驱动架构，所有加密方式都实现为独立插件，位于 `encrypt_plugins/`。加载器会自动发现插件并将其暴露为 `-m` 可用选项；插件可在运行时扩展命令行参数（通过实现 `add_arguments(parser)`）。

## 📝 编译特性

使用 Cargo features 控制编译功能：

```bash
# 示例：启用 IPv4 解密 + CreateThread 运行 + Tick 检测 + 鼠标检测 + 桌面文件检测
cargo build --release --no-default-features \
  --features=decrypt_ipv4,run_create_thread,vm_check_tick,vm_check_mouse,vm_check_desktop_files
```

## 🛠️ 二次开发

### 添加新的加密方式（插件模式）
推荐方式：在 Python 层以插件形式添加新的加密方式，同时在 Rust 层添加对应的解密支持。

步骤（高阶）：
1. 在 `encrypt_plugins/` 下新增一个模块文件，例如 `myplugin.py`。模块需遵循插件约定（下面有示例）。
2. 在 `src/decrypt/` 中添加对应的解密模块（用于运行时在 Loader 中解密），并在 `Cargo.toml` 中为该功能添加 feature（例如 `decrypt_myplugin`）。
3. 在 `config/plugins.json` 中注册该加密方式，填写 `encrypt_arg` 对应的插件名以及 `feature` 对应的 Rust feature。
4. 在 GUI 中新增选项（若需要）或让用户通过命令行选择 `-m myplugin`。

示例：`encrypt_plugins/myplugin.py`
```python
name = 'myplugin'
description = '示例：自定义加密插件'

def add_arguments(parser):
  parser.add_argument('--rounds', type=int, default=3, help='polymorph rounds')

def process(data, args):
  # 返回 bytes（未 base64 编码）
  rounds = getattr(args, 'rounds', 3)
  # 在这里实现加密逻辑，示例为伪代码：
  buf = bytearray(data)
  for r in range(rounds):
    # 修改 buf
    pass
  # 返回最终字节数据
  return bytes(buf)
```

插件约定（两种任选其一）：
- 模块级：导出 `name`(str) 和 `process(data, args)` 函数，可选 `add_arguments(parser)`；
- 类级：导出 `Plugin` 类，实例需具备 `name` 属性与 `process(self,data,args)` 方法，可选 `add_arguments(self,parser)`。

加载器行为：程序启动时会扫描 `encrypt_plugins/` 下的 `.py` 文件，导入符合约定的模块并将 `name` 暴露给 `-m/--method`。如果插件实现 `add_arguments`，只有在该插件被选中时才会调用以避免参数冲突。

### 添加新的运行方式
1. 在 `src/exec/` 中实现执行逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册

### 添加新的内存分配方式
1. 在 `src/alloc_mem/` 中实现分配逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册

### 添加新的VM检测策略
1. 在 `src/guard/` 中实现执行逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册


## 📸 免杀截图

过火绒：
![alt text](static/pass1.png)

过微步：
![alt text](static/pass2.png)

过360：
![alt text](static/pass3.png)


## ⚠️ 免责声明

本工具仅供安全研究和教育目的使用。使用者需遵守当地法律法规，不得用于非法用途。作者不对任何滥用行为承担责任。

## 📄 开源许可

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [JoJoLoader](https://github.com/Pizz33/JoJoLoader) by [@Pizz33](https://github.com/Pizz33)
- [sigthief](https://github.com/secretsquirrel/SigThief) - 签名伪造工具
- Rust 社区
- PyQt5 开发团队

⭐ 如果这个项目对你有帮助，请给一个 Star！