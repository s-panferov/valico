doc:
	git checkout gh-pages
	cargo doc
	cp -r target/doc doc
	echo valico.rustless.org > CNAME
	git add --all
	msg="doc(*): rebuilding docs `date`"
	git commit -m "$msg"
	git push -f origin gh-pages
	git checkout master
