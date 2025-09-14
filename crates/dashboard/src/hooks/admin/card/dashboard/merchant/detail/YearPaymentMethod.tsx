import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useYearPaymentMethodMerchanDetail(toast: any) {
    const {
        findYearPaymentMethodByMerchant,
        yearPaymentMethod,
        loadingYearPaymentMethod,
        setLoadingYearPaymentMethod,
        setErrorYearPaymentMethod,
    } = useMerchantStore();


    const yearlyPaymentMethod = useMemo(() => {
        if (!yearPaymentMethod || !Array.isArray(yearPaymentMethod)) {
            return [];
        }

        const methods = new Set(
            yearPaymentMethod.map((item) => item.payment_method),
        );
        const dataByMethod = Array.from(methods).map((method) => ({
            name: method.toUpperCase(),
            data: Array(5).fill(0),
        }));

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        yearPaymentMethod.forEach((item) => {
            const yearIndex = years.indexOf(Number(item.year));
            const methodIndex = Array.from(methods).indexOf(item.payment_method);
            if (yearIndex !== -1 && methodIndex !== -1) {
                dataByMethod[methodIndex].data[yearIndex] = item.total_amount || 0;
            }
        });

        return dataByMethod;
    }, [yearPaymentMethod]);

    const fetchYearlyPaymentMethod = useCallback(
        async (year: number, merchant_id: number) => {
            try {
                setLoadingYearPaymentMethod(true);
                setErrorYearPaymentMethod(null);

                await findYearPaymentMethodByMerchant(toast, year, merchant_id);
            } catch (error) {
                setErrorYearPaymentMethod("Failed to fetch yearly balance");
            } finally {
                setLoadingYearPaymentMethod(false);
            }
        },
        [
            findYearPaymentMethodByMerchant,
            setLoadingYearPaymentMethod,
            setErrorYearPaymentMethod,
        ],
    );

    const debouncedFetchYearlyPaymentMethod = debounce(
        fetchYearlyPaymentMethod,
        1000,
    );

    return {
        debouncedFetchYearlyPaymentMethod,
        findYearPaymentMethodByMerchant,
        yearPaymentMethod,
        loadingYearPaymentMethod,
        setLoadingYearPaymentMethod,
        setErrorYearPaymentMethod,
        yearlyPaymentMethod,
        fetchYearlyPaymentMethod
    }
}