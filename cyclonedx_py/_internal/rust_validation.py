# This file is part of CycloneDX Python
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0
# Copyright (c) OWASP Foundation. All Rights Reserved.

from __future__ import annotations

from typing import Optional

from cyclonedx.schema import SchemaVersion


def validate_bom_json(output: str, *, spec_version: SchemaVersion) -> Optional[str]:
    """
    Validate a CycloneDX BOM JSON document using the optional Rust backend.

    Returns an error string if invalid, or ``None`` if valid.

    :raises ImportError: if the Rust backend is not installed.
    """
    import cyclonedx_bom_rust
    from cyclonedx.schema._res import (  # noqa: PLC2701  # internal dependency
        BOM_JSON,
        CRYPTOGRAPHY_DEFS,
        JSF,
        SPDX_JSON,
    )

    bom_schema_path = BOM_JSON.get(spec_version)
    if not bom_schema_path:
        raise ValueError(f'Unsupported schema_version: {spec_version!r}')

    return cyclonedx_bom_rust.validate_bom_json(
        output,
        bom_schema_path,
        SPDX_JSON,
        CRYPTOGRAPHY_DEFS,
        JSF,
    )

