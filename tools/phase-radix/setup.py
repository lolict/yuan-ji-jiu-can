"""
满全法 · 相位进制画布 (Phase Radix Canvas)

安装：
  pip install -e tools/phase-radix

使用：
  python -m phase_radix demo
  python -m phase_radix canvas my_canvas --width 32 --height 32
  python -m phase_radix compile my_canvas --format ascii
"""

from setuptools import setup, find_packages

setup(
    name="phase-radix",
    version="0.1.0",
    description="满全法 · 相位进制画布 — 离散相位单元 + 加减运算",
    long_description=open("README.md", encoding="utf-8").read() if __import__("os").path.exists("README.md") else "",
    long_description_content_type="text/markdown",
    author="刘楚恬和我",
    author_email="",
    url="https://github.com/lolict/hunyuan-os",
    packages=find_packages(),
    python_requires=">=3.8",
    install_requires=[
        # 核心功能无依赖，编译图像需要 pillow（可选）
    ],
    extras_require={
        "image": ["pillow>=9.0"],
        "all": ["pillow>=9.0"],
    },
    entry_points={
        "console_scripts": [
            "phase-radix=phase_radix.cli:main",
        ],
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Natural Language :: Chinese (Simplified)",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Software Development :: Libraries",
    ],
)
