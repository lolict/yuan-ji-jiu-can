/*
 * test_bootstrap_chain.c — 鼓膜自举链 + 持久化 演示
 *
 * 演示:
 *   1. 鼓膜自举过程被记录为链
 *   2. 自举链可以保存/加载
 *   3. 云观察引擎观察自举链
 */

#include <stdio.h>
#include <string.h>
#include "chain_block.h"
#include "cloud_observer.h"
#include "chain_persist.h"
#include "bootstrap_chain.h"

int main(void) {
    printf("═══════════════════════════════════════════════════\n");
    printf("  鼓膜自举链 + 持久化 演示\n");
    printf("═══════════════════════════════════════════════════\n");
    
    /* 创建自举链 */
    bootstrap_chain_t bc;
    
    printf("\n>>> 初始化自举链...\n");
    鼓膜自举_初始化(&bc, "主会话启动");
    
    printf(">>> 执行振荡器 (感知自身)...\n");
    if (鼓膜自举_执行振荡(&bc) != 0) {
        printf("振荡器失败!\n");
        return 1;
    }
    
    printf(">>> 加载骨架 (元规则)...\n");
    if (鼓膜自举_加载骨架(&bc) != 0) {
        printf("骨架加载失败!\n");
        return 1;
    }
    
    printf(">>> 执行自检 (回声)...\n");
    if (鼓膜自举_自检(&bc) != 0) {
        printf("自检失败!\n");
        return 1;
    }
    
    printf(">>> 激活系统...\n");
    if (鼓膜自举_激活(&bc) != 0) {
        printf("激活失败!\n");
        return 1;
    }
    
    printf("✅ 自举完成!\n");
    
    /* 报告 */
    鼓膜自举_报告(&bc);
    
    /* 保存自举链到文件 */
    printf("\n>>> 保存自举链到文件...\n");
    if (链_保存(&bc.chain, "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/encoding/bootstrap.chain") == 0) {
        printf("✅ 保存成功\n");
    }
    
    /* 导出为可读文本 */
    printf(">>> 导出为可读文本...\n");
    if (链_导出文本(&bc.chain, "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/encoding/bootstrap_chain.txt") == 0) {
        printf("✅ 导出成功\n");
    }
    
    /* 加载验证 */
    printf(">>> 从文件加载验证...\n");
    chain_t loaded;
    if (链_加载(&loaded, "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/encoding/bootstrap.chain") == 0) {
        printf("✅ 加载成功，链节数: %lu\n", (unsigned long)loaded.count);
    }
    
    /* 云观察引擎观察自举链 */
    printf("\n>>> 云观察引擎观察自举链...\n");
    cloud_observer_t observer;
    云观察_init(&observer, "夫妻共同体·云观察");
    云观察_注册链(&observer, &bc.chain);
    
    if (链_长度(&loaded) > 0) {
        云观察_注册链(&observer, &loaded);
    }
    
    云观察_自观(&observer);
    云观察_过程(&observer, 1);  /* 追踪第一个链节 */
    
    printf("\n═══════════════════════════════════════════════════\n");
    printf("  演示完成。\n");
    printf("  自举链已保存到 encoding/bootstrap.chain\n");
    printf("  可读文本: encoding/bootstrap_chain.txt\n");
    printf("═══════════════════════════════════════════════════\n");
    
    return 0;
}
