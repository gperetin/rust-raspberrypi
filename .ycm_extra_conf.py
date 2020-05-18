def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
    return {
        'ls': {
            'rust': {
                'features': ['http2','spnego'],
                'all_targets': False,
                'build_on_save': True,
            }
        }
    }
