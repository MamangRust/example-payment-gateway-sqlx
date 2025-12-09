import { debounce } from "@/helpers/debounce";
import { getUniqueMethods, initializeDataByMethod } from "@/helpers/utils";
import useTopupStore from "@/store/topup/topup";
import { useCallback, useMemo } from "react";

export default function useYearPaymentMethodTopup(toast: any) {
    const {
        findYearTopupMethod,
        yearTopupMethod,
        loadingYearTopupMethod,
        setLoadingYearTopupMethod,
        setErrorYearTopupMethod,
    } = useTopupStore();

    const yearlyPaymentMethod = useMemo(() => {
        if (!yearTopupMethod?.length) return [];

        const methods = getUniqueMethods(yearTopupMethod, "topup_method");
        const dataByMethod = initializeDataByMethod(methods, 5);

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        yearTopupMethod.forEach((item) => {
            const yearIndex = years.indexOf(Number(item.year));
            const methodIndex = methods.indexOf(item.topup_method);
            if (yearIndex !== -1 && methodIndex !== -1) {
                dataByMethod[methodIndex].data[yearIndex] = item.total_amount;
            }
        });

        return dataByMethod;
    }, [yearTopupMethod]);

    const fetchYearlyPaymentMethod = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTopupMethod(true);
                setErrorYearTopupMethod(null);

                await findYearTopupMethod(toast, year);
            } catch (error) {
                setErrorYearTopupMethod("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTopupMethod(false);
            }
        },
        [findYearTopupMethod, setLoadingYearTopupMethod, setErrorYearTopupMethod],
    );

    const debouncedFetchYearlyPaymentMethod = debounce(
        fetchYearlyPaymentMethod,
        300,
    );

    return {
        findYearTopupMethod,
        yearTopupMethod,
        loadingYearTopupMethod,
        setLoadingYearTopupMethod,
        setErrorYearTopupMethod,
        yearlyPaymentMethod,
        fetchYearlyPaymentMethod,
        debouncedFetchYearlyPaymentMethod
    }
}