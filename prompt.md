这个项目的核心是核心是一个数据表格，用于管理所有的 mod。

mhw 的 mod 的机制：
所有的mod通过压缩包导入到项目。
压缩包里有一个 nativepc 文件夹，nativepc 可能有大小写的变化如 `nativePC`， `NATIVEPC`等等，统一识别为 nativepc。

除了 nativePC 文件夹，在同一层可能还有其他文件，比如一个mod可能是这样的文件层级：
```
- nativepc
    - assets
        - some file
    - pl
        some file
- other dll file
- other dll file
```

mod的安装方式：
有一个游戏的主目录,windows 可能在 `E:\steam\steamapps\common\Monster Hunter World` 这样的位置, linux 可能在 `/home/test/.steam/steam/common/Monster Hunter World` 这样的位置。
用户可以自定义文件夹位置。

然后，压缩包里的 nativepc 需要放到游戏主目录中，也就是
- `E:\steam\steamapps\common\Monster Hunter World\nativepc`
- `/home/test/.steam/steam/common/Monster Hunter World/nativepc`
(统一设置为 nativepc)
其他文件则直接复制到主目录。

基于此，需要一个解压缩的工具，你来决定使用什么工具。

文件可能会有冲突，所以下面的表格来决定冲突怎么解决。



表格需要能 拖动，用来排序 mod，因为有些mod 的文件会冲突，以排在上面的mod为主，（排在1 与 排在8 的mod冲突，1 的文件会覆盖8的文件）
表格展示的信息： 
- mod名称（用户可以修改，只能使用文件夹可用的字符）
- mod id
- 分类
- 状态（是否启用，开关）

文件存储的逻辑：
mod压缩包拖入到mod管理器中，首先要有一个表单，用户可以输入mod相关的信息，比如自定义名称，自定义mod id(可空，唯一)，自定义分类（可空，多选，tag），mod管理器将其解压到数据目录并管理，比如 data 目录，mod使用 mod名称来作为文件夹名称，其下除了有 nativepc 文件夹等mod内容，hd还有一个 mod-info.json 文件作为mod管理器的信息存储文件。
简要流程：
- 用户将mod使用`安装mod按钮`或者`将文件拖入管理器`，触发`安装mod`操作
- 弹出对话框表单，用户可以修改 `名称`(默认填入压缩包名称), `mod id`(默认空)，`分类`（默认空，多选框）。表单之下是一个文件内容预览，展示 nativepc 这些压缩包内的信息。
- 用户点击确认。程序将压缩包解压到 data 目录，将目录名称修改为 `mod名称` 将 `mod id`, `分类`，`是否启用` 写入其下的 mod-info.json
- 用户在数据表格中可以看到mod的信息。