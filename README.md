# **SimpleScheduler** 
 
## **简介** 
 
> ~~是一个运行在用户态的调度器实现~~
 
- ### **什么是`SimpleScheduler`?** 
 
  - `SimpleScheduler`是一个运行在用户态的调度器实现 支持配置不同的调度模式和分应用进行调度切换 
 
 
## **自定义(配置)** 
 
- ### **配置路径: `/data/adb/SimpleScheduler/config.toml`** 
 
- ### **参数(`config`)说明:** 
 
  - **`config`节**
    - `general`: 代表默认情况下应用使用的模式 
      - 可选的值有：`powersave`,`balance`和`performance` 
    - `powersave`: 代表使用`powersave`模式的应用列表 在数组中按照字符串形式提供包名 
      - 示例: `powersave = ["com.example.app1,"com.example.app2"]` 
    - `balance`与`performance`字段的配置规则与`powersave`字段相同 
  - **`freqs`节** 
    - `general`: 该字段无意义  
    - `powersave`: 使用一个具有两个元素的数组来表示当前模式(`powersave`)下的最大和最小频率 ,允许使用Mhz和Khz 
      - 例如`powersave = [1000,1000]` 
    - `balance`与`performance`字段的配置规则与`powersave`字段相同 
  - **`governors`节** 
    - 指定每个模式使用的内核`governor`模式 
    - 如果没有必要 建议保持"walt"即可 
    - ~~也许可以通过将这项的值设置为`scx`来启用风驰调速器？~~ 

## 鸣谢

- [shadow3aaa](https://github.com/shadow3aaa/)
- [grz-1](https://github.com/grz-1)
- [AlexLiuDev233](https://github.com/AlexLiuDev233)

