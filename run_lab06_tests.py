import sys
import pytest

# Ensure week06 directory is importable as top-level module path
sys.path.insert(0, "week06")

if __name__ == '__main__':
    # Run only the lab06 tests
    raise SystemExit(pytest.main(["week06/tests/test_lab06.py", "-q"]))
