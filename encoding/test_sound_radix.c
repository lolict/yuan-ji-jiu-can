#include <stdio.h>
#include <string.h>
#include "sound_radix.h"

int main(void) {
    sound_radix_t sr;
    sound_radix_init(&sr);
    
    printf("─── 声音进制测试 ───\n\n");
    
    /* 编码几个音节 */
    sound_radix_encode_syllable(&sr, "b", "a", 1);  /* bā */
    sound_radix_encode_syllable(&sr, "m", "a", 3);  /* mǎ */
    sound_radix_encode_syllable(&sr, "", "i", 4);   /* ì */
    sound_radix_encode_syllable(&sr, "zh", "ang", 2); /* zháng */
    
    /* 统计 */
    sound_radix_stats(&sr, stdout);
    
    /* WAV 文件头 + PCM 写入 */
    uint32_t duration = sound_radix_get_duration(&sr);
    int sample_rate = sr.sample_rate;
    int n_total = sample_rate * duration / 1000;
    
    float samples[44100 * 3];  /* 最多 3 秒 */
    memset(samples, 0, sizeof(samples));
    int n_rendered = sound_radix_render(&sr, samples, 44100 * 3);
    
    printf("\n─── 渲染结果 ───\n");
    printf("渲染采样数: %d\n", n_rendered);
    printf("采样率: %d Hz\n", sample_rate);
    printf("期望总采样: %d\n", n_total);
    
    /* 写入 WAV 文件 */
    FILE *f = fopen("/tmp/sound_radix_test.wav", "wb");
    if (f) {
        /* WAV 头 */
        uint32_t data_size = n_rendered * 2;  /* 16-bit */
        uint32_t file_size = 36 + data_size;
        uint16_t pcm = 1;
        uint16_t channels = 1;
        uint32_t byte_rate = sample_rate * 2;
        uint16_t block_align = 2;
        uint16_t bits = 16;
        
        fwrite("RIFF", 1, 4, f);
        fwrite(&file_size, 4, 1, f);
        fwrite("WAVE", 1, 4, f);
        fwrite("fmt ", 1, 4, f);
        uint32_t fmt_size = 16; fwrite(&fmt_size, 4, 1, f);
        fwrite(&pcm, 2, 1, f);
        fwrite(&channels, 2, 1, f);
        fwrite(&sample_rate, 4, 1, f);
        fwrite(&byte_rate, 4, 1, f);
        fwrite(&block_align, 2, 1, f);
        fwrite(&bits, 2, 1, f);
        fwrite("data", 1, 4, f);
        fwrite(&data_size, 4, 1, f);
        
        /* 写入 PCM (16-bit signed) */
        for (int i = 0; i < n_rendered; i++) {
            float s = samples[i];
            if (s > 1.0f) s = 1.0f;
            if (s < -1.0f) s = -1.0f;
            int16_t v = (int16_t)(s * 32000);
            fwrite(&v, 2, 1, f);
        }
        fclose(f);
        printf("\n已写入: /tmp/sound_radix_test.wav (%u bytes)", file_size + 8);
    }
    
    printf("\n\n─── 测试完成 ───\n");
    return 0;
}
