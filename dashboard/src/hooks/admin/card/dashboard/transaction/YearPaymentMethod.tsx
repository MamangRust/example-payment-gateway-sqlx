import { debounce } from "@/helpers/debounce";
import { getUniqueMethods, initializeDataByMethod } from "@/helpers/utils";
import useTransactionStore from "@/store/transaction/transaction";
import { useCallback, useMemo } from "react";

export default function useYearPaymentMethodTransaction(toast: any) {
    const {
        findYearTransactionMethod,
        yearTransactionMethod,
        loadingYearTransactionMethod,
        setLoadingYearTransactionMethod,
        setErrorYearTransactionMethod,
    } = useTransactionStore();

    const yearlyPaymentMethod = useMemo(() => {
        if (!yearTransactionMethod?.length) return [];

        const methods = getUniqueMethods(yearTransactionMethod, "payment_method");
        const dataByMethod = initializeDataByMethod(methods, 5);

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        yearTransactionMethod.forEach((item) => {
            const yearIndex = years.indexOf(Number(item.year));
            const methodIndex = methods.indexOf(item.payment_method);
            if (yearIndex !== -1 && methodIndex !== -1) {
                dataByMethod[methodIndex].data[yearIndex] = item.total_amount;
            }
        });

        return dataByMethod;
    }, [yearTransactionMethod]);

    const fetchYearlyPaymentMethod = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTransactionMethod(true);
                setErrorYearTransactionMethod(null);

                await findYearTransactionMethod(toast, year);
            } catch (error) {
                setErrorYearTransactionMethod("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTransactionMethod(false);
            }
        },
        [findYearTransactionMethod, setLoadingYearTransactionMethod, setErrorYearTransactionMethod],
    );

    const debouncedFetchYearlyPaymentMethod = debounce(
        fetchYearlyPaymentMethod,
        300,
    );

    return {
        findYearTransactionMethod,
        yearTransactionMethod,
        loadingYearTransactionMethod,
        setLoadingYearTransactionMethod,
        setErrorYearTransactionMethod,
        yearlyPaymentMethod,
        fetchYearlyPaymentMethod,
        debouncedFetchYearlyPaymentMethod
    }
}