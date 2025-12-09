import { debounce } from "@/helpers/debounce";
import { getUniqueMethods, initializeDataByMethod } from "@/helpers/utils";
import useTransactionStore from "@/store/transaction/transaction";
import { useCallback, useMemo } from "react";


export default function useMonthPaymentMethodTransaction(toast: any) {
    const {
        findMonthTransactionMethod,
        monthTransactionMethod,
        loadingMonthTransactionMethod,
        setLoadingMonthTransactionMethod,
        setErrorMonthTransactionMethod,
    } = useTransactionStore();

    const monthlyPaymentMethod = useMemo(() => {
        if (!monthTransactionMethod?.length) return [];

        const year = new Date().getFullYear();
        const methods = getUniqueMethods(monthTransactionMethod, "payment_method");
        const dataByMethod = initializeDataByMethod(methods, 12);

        monthTransactionMethod.forEach((item) => {
            const monthIndex = new Date(`${item.month} 1, ${year}`).getMonth();
            const methodIndex = methods.indexOf(item.payment_method);
            if (methodIndex !== -1 && item.total_amount) {
                dataByMethod[methodIndex].data[monthIndex] = item.total_amount;
            }
        });

        return dataByMethod;
    }, [monthTransactionMethod]);


    const fetchMonthlyPaymentMethod = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTransactionMethod(true);
                setErrorMonthTransactionMethod(null);

                await findMonthTransactionMethod(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransactionMethod("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransactionMethod(false);
            }
        },
        [
            findMonthTransactionMethod,
            setLoadingMonthTransactionMethod,
            setErrorMonthTransactionMethod,
        ],
    );
    const debouncedFetchMonthlyPaymentMethod = debounce(
        fetchMonthlyPaymentMethod,
        300,
    );

    return {
        findMonthTransactionMethod,
        monthTransactionMethod,
        loadingMonthTransactionMethod,
        setLoadingMonthTransactionMethod,
        setErrorMonthTransactionMethod,
        monthlyPaymentMethod,
        fetchMonthlyPaymentMethod,
        debouncedFetchMonthlyPaymentMethod
    }
}