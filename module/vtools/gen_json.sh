#!/system/bin/sh
#
# Copyright 2023 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

propPath=$1
version=$(cat $propPath | grep "version=" | cut -d "=" -f2)
versionCode=$(cat $propPath | grep "versionCode=" | cut -d "=" -f2)

json=$(
	cat <<EOF
{
    "name": "FAS-RS",
    "author": "shadow3",
    "version": "$version",
    "versionCode": ${versionCode},
    "features": {
        "strict": true,
        "pedestal": true
    },
    "module": "fas_rs",
    "state": "/dev/fas_rs/mode",
    "entry": "/data/powercfg.sh",
    "projectUrl": "https://github.com/shadow3aaa/fas-rs"
}
EOF
)
