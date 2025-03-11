import { init, locale, register } from 'svelte-i18n';

register('pl', () => import('../locales/pl.json'));
register('en', () => import('../locales/en.json'));

init({
	fallbackLocale: 'en',
	initialLocale: 'en'
});

export { locale };
