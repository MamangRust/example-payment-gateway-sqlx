export const getUniqueMethods = (data: any, key: any) => {
    return Array.from(new Set(data.map((item: any) => item[key])));
};

export const initializeDataByMethod = (methods: any, length: any) => {
    return methods.map((method: any) => ({
        name: method.toUpperCase(),
        data: Array(length).fill(0),
    }));
};