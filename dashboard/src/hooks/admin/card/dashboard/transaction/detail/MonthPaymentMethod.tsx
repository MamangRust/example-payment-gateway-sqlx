import { debounce } from "@/helpers/debounce";
import { getUniqueMethods, initializeDataByMethod } from "@/helpers/utils";
import useTransactionStore from "@/store/transaction/transaction";
import { useCallback, useMemo } from "react";


export default function useMonthPaymentMethodTransactionDetail(toast: any) {
    const {
        findMonthTransactionMethodCard,
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
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthTransactionMethod(true);
                setErrorMonthTransactionMethod(null);

                await findMonthTransactionMethodCard(toast, year,card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTransactionMethod("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTransactionMethod(false);
            }
        },
        [
            findMonthTransactionMethodCard,
            setLoadingMonthTransactionMethod,
            setErrorMonthTransactionMethod,
        ],
    );
    const debouncedFetchMonthlyPaymentMethod = debounce(
        fetchMonthlyPaymentMethod,
        300,
    );

    return {
        findMonthTransactionMethodCard,
        monthTransactionMethod,
        loadingMonthTransactionMethod,
        setLoadingMonthTransactionMethod,
        setErrorMonthTransactionMethod,
        monthlyPaymentMethod,
        fetchMonthlyPaymentMethod,
        debouncedFetchMonthlyPaymentMethod
    }
}