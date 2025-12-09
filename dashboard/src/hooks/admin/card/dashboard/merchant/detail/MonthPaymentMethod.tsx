import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useMonthPaymentMethodMerchantDetail(toast: any) {
    const {
        findMonthPaymentMethodByMerchant,
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
        async (year: number, id: number) => {
            try {
                setLoadingMonthPaymentMethod(true);
                setErrorMonthPaymentMethod(null);

                await findMonthPaymentMethodByMerchant(toast, year, id);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthPaymentMethod("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthPaymentMethod(false);
            }
        },
        [
            findMonthPaymentMethodByMerchant,
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
        findMonthPaymentMethodByMerchant,
        monthPaymentMethod,
        loadingMonthPaymentMethod,
        setLoadingMonthPaymentMethod,
        setErrorMonthPaymentMethod,
        monthlyPaymentMethod,
        fetchMonthlyPaymentMethod
    }
}