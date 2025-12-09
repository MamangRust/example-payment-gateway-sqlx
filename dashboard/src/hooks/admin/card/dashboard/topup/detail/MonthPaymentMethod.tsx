import { debounce } from "@/helpers/debounce";
import { getUniqueMethods, initializeDataByMethod } from "@/helpers/utils";
import useTopupStore from "@/store/topup/topup";
import { useCallback, useMemo } from "react";


export default function useMonthPaymentMethodTopupDetail(toast: any) {
    const {
        findMonthTopupMethodCard,
        monthTopupMethod,
        loadingMonthTopupMethod,
        setLoadingMonthTopupMethod,
        setErrorMonthTopupMethod,
    } = useTopupStore();

    const monthlyPaymentMethod = useMemo(() => {
        if (!monthTopupMethod?.length) return [];

        const year = new Date().getFullYear();
        const methods = getUniqueMethods(monthTopupMethod, "topup_method");
        const dataByMethod = initializeDataByMethod(methods, 12);

        monthTopupMethod.forEach((item) => {
            const monthIndex = new Date(`${item.month} 1, ${year}`).getMonth();
            const methodIndex = methods.indexOf(item.topup_method);
            if (methodIndex !== -1 && item.total_amount) {
                dataByMethod[methodIndex].data[monthIndex] = item.total_amount;
            }
        });

        return dataByMethod;
    }, [monthTopupMethod]);


    const fetchMonthlyPaymentMethod = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthTopupMethod(true);
                setErrorMonthTopupMethod(null);

                await findMonthTopupMethodCard(toast, year, card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTopupMethod("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTopupMethod(false);
            }
        },
        [
            findMonthTopupMethodCard,
            setLoadingMonthTopupMethod,
            setErrorMonthTopupMethod,
        ],
    );
    const debouncedFetchMonthlyPaymentMethod = debounce(
        fetchMonthlyPaymentMethod,
        300,
    );

    return {
        findMonthTopupMethodCard,
        monthTopupMethod,
        loadingMonthTopupMethod,
        setLoadingMonthTopupMethod,
        setErrorMonthTopupMethod,
        monthlyPaymentMethod,
        fetchMonthlyPaymentMethod,
        debouncedFetchMonthlyPaymentMethod
    }
}