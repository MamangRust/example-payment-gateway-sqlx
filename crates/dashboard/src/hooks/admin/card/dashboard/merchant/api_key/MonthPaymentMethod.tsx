import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useMonthPaymentMethodMerchantApiKey(toast: any) {
    const {
        findMonthPaymentMethodByApiKey,
        monthPaymentMethod,
        loadingMonthPaymentMethod,
        setLoadingMonthPaymentMethod,
        setErrorMonthPaymentMethod,
    } = useMerchantStore();

    const monthlyPaymentMethod = useMemo(() => {
        if (!monthPaymentMethod || !Array.isArray(monthPaymentMethod)) {
            return [];
        }

        const year = new Date().getFullYear();
        const methods = new Set(
            monthPaymentMethod.map((item) => item.payment_method),
        );
        const dataByMethod = Array.from(methods).map((method) => ({
            name: method.toUpperCase(),
            data: Array(12).fill(0),
        }));

        monthPaymentMethod.forEach((item) => {
            const monthIndex = new Date(`${item.month} 1, ${year}`).getMonth();
            const methodIndex = Array.from(methods).indexOf(item.payment_method);
            if (methodIndex !== -1 && item.total_amount !== undefined) {
                dataByMethod[methodIndex].data[monthIndex] = item.total_amount;
            }
        });

        return dataByMethod;
    }, [monthPaymentMethod]);

    const fetchMonthlyPaymentMethod = useCallback(
        async (year: number, api_key: string) => {
            try {
                setLoadingMonthPaymentMethod(true);
                setErrorMonthPaymentMethod(null);

                await findMonthPaymentMethodByApiKey(toast, year, api_key);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthPaymentMethod("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthPaymentMethod(false);
            }
        },
        [
            findMonthPaymentMethodByApiKey,
            setLoadingMonthPaymentMethod,
            setErrorMonthPaymentMethod,
        ],
    );

    const debouncedFetchMonthlyPaymentMethod = debounce(
        fetchMonthlyPaymentMethod,
        1000,
    );

    return {
        debouncedFetchMonthlyPaymentMethod,
        findMonthPaymentMethodByApiKey,
        monthPaymentMethod,
        loadingMonthPaymentMethod,
        setLoadingMonthPaymentMethod,
        setErrorMonthPaymentMethod,
        monthlyPaymentMethod,
        fetchMonthlyPaymentMethod
    }
}