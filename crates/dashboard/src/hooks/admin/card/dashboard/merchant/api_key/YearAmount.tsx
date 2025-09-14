import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useYearAmountMerchantApiKey(toast: any) {
    const {
        findYearAmountByApiKey,
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
        async (year: number, api_key: string) => {
            try {
                setLoadingYearAmount(true);
                setErrorYearAmount(null);

                await findYearAmountByApiKey(toast, year, api_key);
            } catch (error) {
                setErrorYearAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearAmount(false);
            }
        },
        [findYearAmountByApiKey, setLoadingYearAmount, setErrorYearAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 1000);

    return {
        debouncedFetchYearlyAmount,
        findYearAmountByApiKey,
        yearAmount,
        loadingYearAmount,
        setLoadingYearAmount,
        setErrorYearAmount,
        yearlyAmount,
        fetchYearlyAmount
    }

}