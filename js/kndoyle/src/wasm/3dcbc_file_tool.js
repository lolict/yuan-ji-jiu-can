/**
 * .3d.cbc.mll 文件读写工具
 * 私有格式，请勿外传
 */

class CBCFileReader {
    constructor() {
        this.magic = new Uint8Array([0x3D, 0x43, 0x42, 0x43]); // .3dCBC
        this.version = 0x0200;
    }

    read(buffer) {
        const view = new DataView(buffer);
        let offset = 0;

        // 验证魔数
        if (!this.verifyMagic(view, offset)) {
            throw new Error('Invalid .3d.cbc.mll file');
        }
        offset += 4;

        // 读取版本
        const fileVersion = view.getUint16(offset, false);
        offset += 2;
        if (fileVersion < this.version) {
            throw new Error('Unsupported file version');
        }

        // 读取标志位
        const flags = view.getUint16(offset, false);
        offset += 2;

        // 读取数据偏移
        const dataOffset = view.getUint32(offset, false);
        offset += 4;

        // 读取校验和
        const checksum = view.getUint32(offset, false);
        offset += 4;

        // 验证校验和
        if (!this.verifyChecksum(buffer, checksum)) {
            throw new Error('Checksum mismatch');
        }

        // 读取数据段
        const segments = [];
        while (offset < buffer.byteLength) {
            if (offset + 8 > buffer.byteLength) break;

            const segType = view.getUint16(offset, false);
            const segFlags = view.getUint16(offset + 2, false);
            const segLength = view.getUint32(offset + 4, false);
            offset += 8;

            if (segType === 0xFF) break; // END标记

            const segData = new Uint8Array(buffer, offset, segLength);
            offset += segLength;

            segments.push({
                type: segType,
                flags: segFlags,
                data: segData
            });
        }

        return this.parseSegments(segments);
    }

    verifyMagic(view, offset) {
        for (let i = 0; i < 4; i++) {
            if (view.getUint8(offset + i) !== this.magic[i]) return false;
        }
        return true;
    }

    verifyChecksum(buffer, expected) {
        let checksum = 0;
        for (let i = 0; i < buffer.byteLength - 16; i++) {
            checksum ^= buffer[i];
        }
        return checksum === expected;
    }

    parseSegments(segments) {
        const result = {
            radix: [],
            charset: [],
            containers: null,
            spatial: null,
            timenodes: [],
            plugins: []
        };

        for (const seg of segments) {
            switch (seg.type) {
                case 0x01: // RADIX
                    result.radix = this.parseRadix(seg.data);
                    break;
                case 0x02: // CHARSET
                    result.charset = this.parseCharset(seg.data);
                    break;
                case 0x03: // CONTAINER
                    result.containers = this.parseContainer(seg.data);
                    break;
                case 0x04: // SPATIAL
                    result.spatial = this.parseSpatial(seg.data);
                    break;
                case 0x05: // TIMENODE
                    result.timenodes = this.parseTimeNode(seg.data);
                    break;
                case 0x06: // PLUGIN
                    result.plugins = this.parsePlugin(seg.data);
                    break;
            }
        }

        return result;
    }

    parseRadix(data) {
        const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
        const count = view.getUint8(0);
        const radixes = [];

        for (let i = 0; i < count; i++) {
            const offset = 1 + i * 6;
            radixes.push({
                id: view.getUint8(offset),
                base: view.getUint16(offset + 1, false),
                min: view.getUint16(offset + 3, false),
                flags: view.getUint8(offset + 5)
            });
        }

        return radixes;
    }

    parseCharset(data) {
        const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
        const count = view.getUint16(0, false);
        const mappings = [];

        for (let i = 0; i < count; i++) {
            const offset = 2 + i * 4;
            mappings.push({
                source: view.getUint8(offset),
                custom: view.getUint16(offset + 1, false),
                container: view.getUint8(offset + 3)
            });
        }

        return mappings;
    }

    parseContainer(data) {
        return {
            values: new Uint8Array(data.buffer, data.byteOffset, 256),
            positions: new Uint8Array(data.buffer, data.byteOffset + 256, 1024),
            flags: new Uint8Array(data.buffer, data.byteOffset + 1280, 256)
        };
    }

    parseSpatial(data) {
        const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
        return {
            type: view.getUint8(0),
            dimension: view.getUint8(1),
            origin: {
                x: view.getInt32(2, false),
                y: view.getInt32(6, false),
                z: view.getInt32(10, false)
            },
            elementCount: view.getUint16(14, false)
        };
    }

    parseTimeNode(data) {
        const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
        const count = view.getUint16(0, false);
        const nodes = [];

        for (let i = 0; i < count; i++) {
            const offset = 2 + i * 12;
            const causeId = view.getUint32(offset + 4, false);
            nodes.push({
                id: view.getUint32(offset, false),
                type: view.getUint8(offset + 4),
                cause: causeId === 0xFFFFFFFF ? null : causeId,
                offset: view.getUint16(offset + 9, false),
                flags: view.getUint8(offset + 11)
            });
        }

        return nodes;
    }

    parsePlugin(data) {
        const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
        const count = view.getUint16(0, false);
        const plugins = [];

        for (let i = 0; i < count; i++) {
            const offset = 2 + i * 20;
            const nameBytes = new Uint8Array(data.buffer, data.byteOffset + offset + 1, 16);
            plugins.push({
                id: view.getUint8(offset),
                name: this.bytesToString(nameBytes).trim(),
                version: view.getUint16(offset + 17, false),
                status: view.getUint8(offset + 19)
            });
        }

        return plugins;
    }

    bytesToString(bytes) {
        let str = '';
        for (const b of bytes) {
            if (b === 0) break;
            str += String.fromCharCode(b);
        }
        return str;
    }
}

class CBCFileWriter {
    constructor() {
        this.magic = new Uint8Array([0x3D, 0x43, 0x42, 0x43]);
        this.version = 0x0200;
    }

    write(data) {
        const segments = this.buildSegments(data);
        const totalSize = this.calculateSize(segments);
        
        const buffer = new ArrayBuffer(totalSize);
        const view = new DataView(buffer);
        let offset = 0;

        // 写入魔数
        for (let i = 0; i < 4; i++) {
            view.setUint8(offset++, this.magic[i]);
        }

        // 写入版本
        view.setUint16(offset, this.version, false);
        offset += 2;

        // 写入标志位
        let flags = 0;
        if (data.charset && data.charset.length > 0) flags |= 0x04;
        if (data.spatial) flags |= 0x08;
        if (data.timenodes && data.timenodes.length > 0) flags |= 0x10;
        view.setUint16(offset, flags, false);
        offset += 2;

        // 写入数据偏移 (16)
        view.setUint32(offset, 16, false);
        offset += 4;

        // 先留空校验和
        const checksumOffset = offset;
        view.setUint32(offset, 0, false);
        offset += 4;

        // 写入段数据
        for (const seg of segments) {
            view.setUint16(offset, seg.type, false);
            view.setUint16(offset + 2, seg.flags, false);
            view.setUint32(offset + 4, seg.data.length, false);
            offset += 8;

            for (let i = 0; i < seg.data.length; i++) {
                view.setUint8(offset++, seg.data[i]);
            }
        }

        // 写入结束标记
        view.setUint16(offset, 0xFF, false);
        view.setUint16(offset + 2, 0, false);
        view.setUint32(offset + 4, 0, false);
        offset += 8;

        // 计算并写入校验和
        const checksum = this.calculateChecksum(buffer, 0, offset - 16);
        view.setUint32(checksumOffset, checksum, false);

        return buffer;
    }

    buildSegments(data) {
        const segments = [];

        if (data.radix && data.radix.length > 0) {
            segments.push(this.buildRadixSegment(data.radix));
        }

        if (data.charset && data.charset.length > 0) {
            segments.push(this.buildCharsetSegment(data.charset));
        }

        if (data.containers) {
            segments.push(this.buildContainerSegment(data.containers));
        }

        if (data.spatial) {
            segments.push(this.buildSpatialSegment(data.spatial));
        }

        if (data.timenodes && data.timenodes.length > 0) {
            segments.push(this.buildTimeNodeSegment(data.timenodes));
        }

        if (data.plugins && data.plugins.length > 0) {
            segments.push(this.buildPluginSegment(data.plugins));
        }

        return segments;
    }

    buildRadixSegment(radixes) {
        const size = 1 + radixes.length * 6;
        const buffer = new Uint8Array(size);
        const view = new DataView(buffer.buffer);

        buffer[0] = radixes.length;
        for (let i = 0; i < radixes.length; i++) {
            const offset = 1 + i * 6;
            view.setUint8(offset, radixes[i].id);
            view.setUint16(offset + 1, radixes[i].base, false);
            view.setUint16(offset + 3, radixes[i].min, false);
            view.setUint8(offset + 5, radixes[i].flags || 0);
        }

        return { type: 0x01, flags: 0, data: buffer };
    }

    buildCharsetSegment(mappings) {
        const size = 2 + mappings.length * 4;
        const buffer = new Uint8Array(size);
        const view = new DataView(buffer.buffer);

        view.setUint16(0, mappings.length, false);
        for (let i = 0; i < mappings.length; i++) {
            const offset = 2 + i * 4;
            view.setUint8(offset, mappings[i].source);
            view.setUint16(offset + 1, mappings[i].custom, false);
            view.setUint8(offset + 3, mappings[i].container);
        }

        return { type: 0x02, flags: 0, data: buffer };
    }

    buildContainerSegment(containers) {
        const buffer = new Uint8Array(1536);
        if (containers.values) {
            buffer.set(containers.values, 0);
        }
        if (containers.positions) {
            buffer.set(containers.positions.slice(0, 1024), 256);
        }
        if (containers.flags) {
            buffer.set(containers.flags, 1280);
        }

        return { type: 0x03, flags: 0, data: buffer };
    }

    buildSpatialSegment(spatial) {
        const buffer = new Uint8Array(16);
        const view = new DataView(buffer.buffer);

        view.setUint8(0, spatial.type);
        view.setUint8(1, spatial.dimension);
        view.setInt32(2, spatial.origin.x, false);
        view.setInt32(6, spatial.origin.y, false);
        view.setInt32(10, spatial.origin.z, false);
        view.setUint16(14, spatial.elementCount || 0, false);

        return { type: 0x04, flags: 0, data: buffer };
    }

    buildTimeNodeSegment(nodes) {
        const size = 2 + nodes.length * 12;
        const buffer = new Uint8Array(size);
        const view = new DataView(buffer.buffer);

        view.setUint16(0, nodes.length, false);
        for (let i = 0; i < nodes.length; i++) {
            const offset = 2 + i * 12;
            view.setUint32(offset, nodes[i].id, false);
            view.setUint8(offset + 4, nodes[i].type);
            view.setUint32(offset + 5, nodes[i].cause || 0xFFFFFFFF, false);
            view.setUint16(offset + 9, nodes[i].offset || 0, false);
            view.setUint8(offset + 11, nodes[i].flags || 0);
        }

        return { type: 0x05, flags: 0, data: buffer };
    }

    buildPluginSegment(plugins) {
        const size = 2 + plugins.length * 20;
        const buffer = new Uint8Array(size);

        const view = new DataView(buffer.buffer);
        view.setUint16(0, plugins.length, false);

        for (let i = 0; i < plugins.length; i++) {
            const offset = 2 + i * 20;
            view.setUint8(offset, plugins[i].id);
            
            const name = plugins[i].name.padEnd(16, '\0');
            for (let j = 0; j < 16; j++) {
                buffer[offset + 1 + j] = name.charCodeAt(j);
            }
            
            view.setUint16(offset + 17, plugins[i].version, false);
            view.setUint8(offset + 19, plugins[i].status || 0);
        }

        return { type: 0x06, flags: 0, data: buffer };
    }

    calculateSize(segments) {
        let size = 16; // 头
        for (const seg of segments) {
            size += 8 + seg.data.length; // 段头 + 数据
        }
        size += 8; // 结束标记
        return size;
    }

    calculateChecksum(buffer, start, end) {
        let checksum = 0;
        const bytes = new Uint8Array(buffer);
        for (let i = start; i < end; i++) {
            checksum ^= bytes[i];
        }
        return checksum;
    }
}

// 导出
window.CBCFileReader = CBCFileReader;
window.CBCFileWriter = CBCFileWriter;

// 测试函数
function testCBCFile() {
    const writer = new CBCFileWriter();
    const data = {
        radix: [{ id: 1, base: 64, min: 0, flags: 0 }],
        charset: [{ source: 65, custom: 1, container: 0x10 }],
        containers: {
            values: new Uint8Array(256),
            positions: new Uint8Array(1024),
            flags: new Uint8Array(256)
        },
        spatial: {
            type: 0x01,
            dimension: 3,
            origin: { x: 0, y: 0, z: 0 },
            elementCount: 100
        },
        timenodes: [{ id: 1, type: 1, cause: null, offset: 0, flags: 0 }],
        plugins: [{ id: 1, name: 'test', version: 1, status: 1 }]
    };

    const buffer = writer.write(data);
    console.log('Written .3d.cbc.mll file:', buffer.byteLength, 'bytes');

    const reader = new CBCFileReader();
    const parsed = reader.read(buffer);
    console.log('Parsed file:', parsed);
}

console.log('.3d.cbc.mll File Tool loaded');
