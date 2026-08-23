from setuptools import setup, find_packages

setup(
    name="color-base",
    version="1.0.0",
    description="颜色进制 · 相位编码系统 — 可编译+可解释+可打包",
    author="lolict",
    package_dir={"": "src"},
    packages=find_packages(where="src"),
    python_requires=">=3.10",
    entry_points={
        "console_scripts": [
            "colorbase=color_base.cli:main",
        ],
    },
)
