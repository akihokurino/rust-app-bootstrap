'use strict';

const { S3Client, GetObjectCommand } = require('@aws-sdk/client-s3');
const sharp = require('sharp');

const s3 = new S3Client();
const MAX_SIZE = 1200;

const errorResponse = (response, status, message) => {
    response.status = status;
    response.headers['content-type'] = [{ key: 'Content-Type', value: 'text/plain' }];
    response.body = message;
    return response;
};

const parseSize = (value) => {
    const num = parseInt(value, 10);
    return Number.isFinite(num) && num > 0 && num <= MAX_SIZE ? num : null;
};

exports.handler = async (event) => {
    const { request, response } = event.Records[0].cf;

    if (response.status === '304') return response;
    if (response.status !== '200') return errorResponse(response, '404', `${request.uri} is not found.`);

    const params = new URLSearchParams(request.querystring);
    const w = params.get('w');
    const h = params.get('h');

    if (!w && !h) return response;

    const width = parseSize(w);
    const height = parseSize(h);

    if ((w && !width) || (h && !height)) {
        return errorResponse(response, '400', `Size must be 1-${MAX_SIZE}.`);
    }

    const targetSize = width || height;

    try {
        const bucketName = request.origin.s3.domainName.split('.')[0];
        const key = decodeURIComponent(request.uri).substring(1);

        const { Body } = await s3.send(new GetObjectCommand({ Bucket: bucketName, Key: key }));
        const imageBuffer = Buffer.from(await Body.transformToByteArray());

        const buffer = await sharp(imageBuffer)
            .rotate()
            .resize(targetSize, targetSize, { fit: 'outside', withoutEnlargement: true })
            .webp()
            .toBuffer();

        response.status = '200';
        response.headers['content-type'] = [{ key: 'Content-Type', value: 'image/webp' }];
        response.body = buffer.toString('base64');
        response.bodyEncoding = 'base64';
        return response;
    } catch (error) {
        console.error('Error:', error);
        return errorResponse(response, '503', 'Internal server error');
    }
};
