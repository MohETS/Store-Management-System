import http from 'k6/http';
import {sleep} from 'k6';
import {Counter, Rate, Trend} from 'k6/metrics';

// Custom metrics for different tests (separate for each route)
let getProductsResponseTime = new Trend('get_products_response_time');
let getProductsErrorRate = new Rate('get_products_error_rate');
let getProductsRequestCount = new Counter('get_products_requests');

let productsUpdateResponseTime = new Trend('product_update_response_time');
let productsUpdateErrorRate = new Rate('product_update_error_rate');
let productsUpdateRequestCount = new Counter('product_update_requests');


export default function () {
    load_testing_get_products()
    load_testing_update_products()
}

export let options = {
    scenarios: {
        get_products_test: {
            executor: 'ramping-vus',
            startTime: '0s',
            stages: [
                { duration: '1m', target: 2000 },
            ],
            exec: 'load_testing_get_products',
        },
        update_products_test: {
            executor: 'ramping-vus',
            startTime: '1m',  // Starts after first test finishes
            stages: [
                { duration: '1m', target: 1000 },
            ],
            exec: 'load_testing_update_products',
        },
        both_tests: {
            executor: 'ramping-vus',
            startTime: '2m',  // Starts after first test finishes
            stages: [
                { duration: '1m', target: 2000 },
            ]
        },

    },
    thresholds: {
        'get_products_response_time': [{ threshold: 'p(95)<1000', abortOnFail: false }],
        'get_products_requests': ['rate > 10'],
        'get_products_error_rate': [{ threshold: 'rate<0.1', abortOnFail: false }],

        'product_update_response_time': [{ threshold: 'p(95)<1000', abortOnFail: false }],
        'product_update_requests': ['rate > 10'],
        'product_update_error_rate': [{ threshold: 'rate<0.1', abortOnFail: false }],
    }
};

export function load_testing_get_products () {
    const url = 'http://localhost:8000/products';
    const res = http.get(url);

    getProductsResponseTime.add(res.timings.duration, { endpoint: '/products' });

    const isError = res.status >= 400 && res.status < 600;
    if (isError) {
        getProductsErrorRate.add(1, { endpoint: '/products' });
    }else{
        getProductsErrorRate.add(0, { endpoint: '/products' });
    }

    getProductsRequestCount.add(1, { endpoint: '/products' });

    sleep(1);
}



const productPresets = [
    { id: 1, name: "Sifu", category: "Games", quantity: 100, price: 25 },
    { id: 1, name: "Sifu", category: "Games", quantity: 350, price: 50 },
    { id: 1, name: "Sifu", category: "Games", quantity: 50, price: 30 },
    { id: 2, name: "Sony XM4", category: "Sound", quantity: 50, price: 250 },
    { id: 2, name: "Sony XM4", category: "Sound", quantity: 300, price: 500 },
    { id: 2, name: "Sony XM4", category: "Sound", quantity: 150, price: 125 },
    { id: 3, name: "Playstation 5", category: "Games", quantity: 75, price: 500 },
    { id: 3, name: "Playstation 5", category: "Games", quantity: 20, price: 425 },
    { id: 3, name: "Playstation 5", category: "Games", quantity: 10, price: 350 },
    { id: 4, name: "FL Studio", category: "Software", quantity: 20, price: 250 },
    { id: 4, name: "FL Studio", category: "Software", quantity: 100, price: 100 },
    { id: 4, name: "FL Studio", category: "Software", quantity: 75, price: 150 },
    { id: 5, name: "NVIDIA GeForce RTX 3080 Ti", category: "Hardware", quantity: 1, price: 900 },
    { id: 5, name: "NVIDIA GeForce RTX 3080 Ti", category: "Hardware", quantity: 10, price: 1000 },
    { id: 5, name: "NVIDIA GeForce RTX 3080 Ti", category: "Hardware", quantity: 0, price: 950 },
];

function getRandomPreset() {
    const index = Math.floor(Math.random() * productPresets.length);
    return productPresets[index];
}

export function load_testing_update_products () {
    const url = 'http://localhost:8000/products/update';
    const productData = getRandomPreset();

    const payload = JSON.stringify(productData);

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const res = http.post(url, payload, params);

    productsUpdateResponseTime.add(res.timings.duration, { endpoint: '/products/update' });

    const isError = res.status >= 400 && res.status < 600;
    if (isError) {
        productsUpdateErrorRate.add(1, { endpoint: '/products/update' });
    }else{
        productsUpdateErrorRate.add(0, { endpoint: '/products/update' });
    }

    productsUpdateRequestCount.add(1, { endpoint: '/products/update' });

    sleep(1);
}