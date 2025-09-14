import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useYearAmountMerchant(toast: any) {
    const {
        findYearAmount,
        yearAmount,
        loadingYearAmount,
        setLoadingYearAmount,
        setErrorYearAmount,
    } = useMerchantStore();

    const yearlyAmount = useMemo(() => {
        if (!yearAmount || !Array.isArray(yearAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingYearAmount(true);
                setErrorYearAmount(null);

                await findYearAmount(toast, year);
            } catch (error) {
                setErrorYearAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearAmount(false);
            }
        },
        [findYearAmount, setLoadingYearAmount, setErrorYearAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 1000);

    return {
        debouncedFetchYearlyAmount,
        findYearAmount,
        yearAmount,
        loadingYearAmount,
        setLoadingYearAmount,
        setErrorYearAmount,
        yearlyAmount,
        fetchYearlyAmount
    }

}