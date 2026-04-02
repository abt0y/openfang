from setuptools import setup

setup(
    name="tapthe-ai",
    version="0.1.0",
    description="Official Python client for the Tapthe.ai Agent OS REST API",
    py_modules=["tapthe_ai_sdk", "tapthe_ai_client"],
    python_requires=">=3.8",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
